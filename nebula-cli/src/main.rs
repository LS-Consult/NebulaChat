mod app;

#[tokio::main]
pub async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    
    let app_result = app::App::default().run(&mut terminal).await; 
    
    ratatui::restore();
    app_result
}
