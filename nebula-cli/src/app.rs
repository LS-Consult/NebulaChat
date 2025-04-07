use ratatui::{DefaultTerminal, Frame};

#[derive(Default)]
pub struct App {
    counter: u64,
    should_exit: bool,
}

impl App {
    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        Ok(())
    }
    
    fn draw(&self, frame: &mut Frame<'_>) {}
}