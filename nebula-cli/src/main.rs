mod app;
mod buffer;
mod cli;

use crate::cli::Cli;
use clap::Parser;
use nebula_common::net::arti::{ArtiConnector, TorTriggerEvent};
use std::sync::Arc;
use tokio::sync::oneshot::Sender;
use tokio::try_join;

#[tokio::main]
pub async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // First, parse any Clap command
    let cli = Cli::try_parse();
    if let Ok(cli) = cli {
        match cli.command {
            cli::Command::Initialize { relay_url } => cli::command::initialize(relay_url)?,
            cli::Command::Auth => cli::command::auth()?,
        }

        return Ok(());
    }

    // If no Clap command is found, bootstrap Tor and initialize Ratatui
    let arti_connector = Arc::new(ArtiConnector::try_new().await?);
    let (tor_trigger_tx, tor_trigger_rx) = tokio::sync::oneshot::channel::<TorTriggerEvent>();

    let tor_service_handle = tokio::spawn(bootstrap_tor(arti_connector.clone(), tor_trigger_tx));
    let application_handle = tokio::spawn(async move {
        let mut ratatui_terminal = ratatui::init();
        let mut application = app::App::new(arti_connector.clone(), tor_trigger_rx);

        application.run(&mut ratatui_terminal).await
    });

    let _ = try_join!(tor_service_handle, application_handle);

    ratatui::restore();
    Ok(())
}

async fn bootstrap_tor(
    arti_connector: Arc<ArtiConnector>,
    tx: Sender<TorTriggerEvent>,
) -> color_eyre::Result<()> {
    arti_connector.start_hidden_service(tx).await?;
    Ok(())
}
