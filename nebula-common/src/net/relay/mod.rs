mod server;

use crate::error::*;
use crate::net::arti::{ArtiConnector, ArtiEvent};
use crate::net::event::RelayEvent;
use ed25519_dalek::ed25519::signature::rand_core::OsRng;
use tokio::task::JoinHandle;

#[derive(Clone)]
pub struct Relay {
    connector: ArtiConnector,
    events_emitter: tokio::sync::broadcast::Sender<RelayEvent>,
}

impl Relay {
    pub async fn try_new() -> Result<Self> {
        let connector = ArtiConnector::try_new().await?;
        let (event_emitter, _) = tokio::sync::broadcast::channel::<RelayEvent>(1000);

        Ok(Self {
            connector,
            events_emitter: event_emitter,
        })
    }

    pub async fn start_relay(
        &self,
        arti_event_trigger: tokio::sync::oneshot::Sender<ArtiEvent>,
    ) -> Result<(JoinHandle<()>, JoinHandle<()>)> {
        let connector = self.connector.clone();
        
        // TODO: remove this hard-coded keys
        let relay_keys = ed25519_dalek::SigningKey::generate(&mut OsRng);

        let arti_task_handle = tokio::spawn(async move {
            connector
                .start_hidden_service(arti_event_trigger)
                .await
                .unwrap();
        });

        let relay_server = server::RelayServer::new();
        let event_emitter = self.events_emitter.clone();
        let relay_task_handle = tokio::spawn(async move {
            relay_server.start(relay_keys, event_emitter).await.unwrap();
        });

        Ok((arti_task_handle, relay_task_handle))
    }
}
