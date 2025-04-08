use crate::error::{ConfigError, Result, SystemError};

use arti_client::config::onion_service::{OnionServiceConfig, OnionServiceConfigBuilder};
use arti_client::{TorClient, TorClientConfig};
use tor_rtcompat::PreferredRuntime;

pub struct ArtiConnector {
    tor: TorClient<PreferredRuntime>,
}

impl ArtiConnector {
    pub async fn try_new() -> Result<Self> {
        let config = TorClientConfig::default();

        TorClient::create_bootstrapped(config)
            .await
            .map_err(|e| SystemError::Arti(e).into())
            .map(|tor| Self { tor })
    }

    pub fn setup_hidden_service(&self) -> Result<OnionServiceConfig> {
        OnionServiceConfigBuilder::default()
            .nickname("nebula_chat".parse().unwrap())
            .enable_pow(true)
            .build()
            .map_err(|_| {
                eprintln!("Failed to build onion service config");
                ConfigError::Arti.into()
            })
    }

    pub fn get_tor(&self) -> &TorClient<PreferredRuntime> {
        &self.tor
    }
}
