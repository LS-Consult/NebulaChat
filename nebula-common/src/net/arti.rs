use crate::error::{ConfigError, Result, SystemError};
use std::net::SocketAddr;
use std::str::FromStr;

use arti_client::config::onion_service::OnionServiceConfigBuilder;
use arti_client::{TorClient, TorClientConfig};
use tokio::sync::oneshot::Sender;
use tor_hsrproxy::config::Encapsulation::Simple;
use tor_hsrproxy::config::{
    ProxyAction, ProxyConfigBuilder, ProxyPattern, ProxyRule, ProxyRuleListBuilder, TargetAddr,
};
use tor_hsrproxy::OnionServiceReverseProxy;
use tor_rtcompat::PreferredRuntime;

#[derive(Clone)]
pub struct ArtiConnector {
    tor: TorClient<PreferredRuntime>,
}

pub enum TorTriggerEvent {
    Running,
    Failed,
}

impl ArtiConnector {
    pub async fn try_new() -> Result<Self> {
        let config = TorClientConfig::default();

        let tor_client = TorClient::builder()
            .config(config)
            .create_bootstrapped()
            .await
            .map_err(SystemError::Arti)?;

        Ok(Self { tor: tor_client })
    }

    pub async fn start_hidden_service(&self, tx: Sender<TorTriggerEvent>) -> Result<()> {
        let onion_service_config = OnionServiceConfigBuilder::default()
            .nickname("nebula_chat".parse().unwrap())
            .enable_pow(true)
            .build()
            .map_err(|_| {
                eprintln!("Failed to build onion service config");
                ConfigError::Arti
            })?;

        let (_, rend_stream) = self
            .tor
            .launch_onion_service(onion_service_config)
            .expect("Failed to launch onion service");

        Self::start_reverse_proxy(rend_stream, tx).await
    }

    async fn start_reverse_proxy(
        rend_stream: impl futures::Stream<Item = tor_hsservice::RendRequest>
        + Unpin
        + Send
        + Sync
        + 'static,
        tx: Sender<TorTriggerEvent>,
    ) -> Result<()> {
        let mut proxy_rule_list = ProxyRuleListBuilder::default();
        proxy_rule_list.access().push(ProxyRule::new(
            ProxyPattern::one_port(80).unwrap(),
            ProxyAction::Forward(
                Simple,
                TargetAddr::Inet(SocketAddr::from_str("127.0.0.1:23690").unwrap()),
            ),
        ));

        let proxy_rule_list = proxy_rule_list
            .build()
            .expect("Failed to build a proxy rule list");

        let mut reverse_proxy_config = ProxyConfigBuilder::default();
        reverse_proxy_config.set_proxy_ports(proxy_rule_list);

        let reverse_proxy_config = reverse_proxy_config
            .build()
            .expect("Failed to build a reverse proxy config");

        let reverse_proxy = OnionServiceReverseProxy::new(reverse_proxy_config);

        tx.send(TorTriggerEvent::Running).map_err(|_| SystemError::ArtiReverseProxy)?;
        
        reverse_proxy
            .handle_requests(
                PreferredRuntime::current().unwrap(),
                "nebula_chat".parse().unwrap(),
                rend_stream,
            )
            .await
            .map_err(|e| {
                eprintln!("Failed to handle rend requests: {}", e);
                SystemError::ArtiReverseProxy
            })?;

        Ok(())
    }

    pub fn get_tor(&self) -> &TorClient<PreferredRuntime> {
        &self.tor
    }
}
