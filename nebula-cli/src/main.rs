mod app;
mod buffer;
mod cli;

use crate::cli::Cli;
use clap::Parser;
use nebula_common::error::NebulaError;
use nebula_common::net::arti::ArtiEvent;
use nebula_common::net::relay::Relay;
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
    let (arti_trigger_tx, arti_trigger_rx) = tokio::sync::oneshot::channel::<ArtiEvent>();
    let (exit_trigger_tx, exit_trigger_rx) = tokio::sync::oneshot::channel::<()>();

    let relay_service_handle = tokio::spawn(async move {
        let relay = Relay::try_new().await.unwrap();
        let (arti_handle, relay_handle) = relay.start_relay(arti_trigger_tx).await.unwrap();
        tokio::select! {
            _ = arti_handle => {
                Ok::<(), NebulaError>(())
            }
            _ = relay_handle => {
                Ok::<(), NebulaError>(())
            }
            _ = exit_trigger_rx => {
                Ok::<(), NebulaError>(())
            }
        }
    });

    let mut ratatui_terminal = ratatui::init();
    let mut application = app::App::new(arti_trigger_rx);

    let result = application.run(&mut ratatui_terminal).await;
    exit_trigger_tx.send(()).unwrap();

    let _ = try_join!(relay_service_handle);

    ratatui::restore();
    result
}
