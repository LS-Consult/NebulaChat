use crate::error::*;
use crate::net::event::RelayEvent;

#[derive(Clone)]
pub struct RelayServer;

impl RelayServer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn start(
        &self,
        relay_keys: ed25519_dalek::SigningKey,
        event_emitter: tokio::sync::broadcast::Sender<RelayEvent>,
    ) -> Result<()> {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:23690")
            .await
            .map_err(SystemError::StdIo)?;

        listener.set_ttl(128).map_err(SystemError::StdIo)?;

        loop {
            let (stream, _) = listener.accept().await.map_err(SystemError::StdIo)?;
            stream.set_nodelay(true).map_err(SystemError::StdIo)?;

            let event_emitter = event_emitter.clone();
            let relay_keys = relay_keys.clone();
            tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            });
        }
    }
}
