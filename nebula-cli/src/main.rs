mod app;

use nebula_common::net::arti::ArtiConnector;
use nebula_common::{futures, tor_hsservice};
use std::sync::Arc;

#[tokio::main]
pub async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Bootstrap the Tor Router
    let (arti_connector, onion_service, request_stream) = bootstrap_tor().await?;

    //Initialize Ratatui
    let mut ratatui_terminal = ratatui::init();
    let mut application = app::App::new(
        false,
        Box::pin(request_stream),
        onion_service,
        arti_connector,
    );

    let application_result = application.run(&mut ratatui_terminal);

    ratatui::restore();
    application_result.await
}

async fn bootstrap_tor() -> color_eyre::Result<(
    ArtiConnector,
    Arc<tor_hsservice::RunningOnionService>,
    impl futures::Stream<Item = tor_hsservice::RendRequest>,
)> {
    println!("🌎 Bootstrapping Tor...");

    let arti_connector = ArtiConnector::try_new().await?;
    let hidden_service_config = arti_connector.setup_hidden_service()?;
    let (onion_service, request_stream) = arti_connector
        .get_tor()
        .launch_onion_service(hidden_service_config)?;

    println!("🛜 Tor bootstrapped!");
    println!(
        "🗺️ Your hidden service is available at: {}",
        onion_service.onion_address().unwrap()
    );
    Ok((arti_connector, onion_service, request_stream))
}
