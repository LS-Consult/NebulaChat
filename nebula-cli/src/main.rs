mod app;
mod buffer;
mod cli;

use crate::cli::Cli;
use clap::Parser;
use nebula_common::net::arti::{ArtiConnector, ArtiTriggerEvent};
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

    // If no Clap command is found, bootstrap Arti and initialize Ratatui
    let (arti_trigger_tx, arti_trigger_rx) = tokio::sync::oneshot::channel::<ArtiTriggerEvent>();

    let tor_service_handle = tokio::spawn(async move {
        let arti_connector = ArtiConnector::try_new().await?;
        arti_connector
            .start_hidden_service(arti_trigger_tx)
            .await
            .map_err(|e| color_eyre::eyre::eyre!(e))
    });

    let application_handle = tokio::spawn(async move {
        let mut ratatui_terminal = ratatui::init();
        let mut application = app::App::new(arti_trigger_rx);

        application.run(&mut ratatui_terminal).await
    });

    let _ = try_join!(tor_service_handle, application_handle);

    ratatui::restore();
    Ok(())
}
