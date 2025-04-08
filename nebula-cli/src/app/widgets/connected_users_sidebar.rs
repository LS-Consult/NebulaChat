use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::widgets::{Block, BorderType, Widget};

#[derive(Debug, Clone)]
pub struct ConnectedUsersSidebarWidget;

impl Widget for &ConnectedUsersSidebarWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered()
            .title("Connected Users")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        block.render(area, buf);
    }
}
