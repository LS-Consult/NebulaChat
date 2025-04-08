use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::Style;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, BorderType, Widget};

pub struct MainChatWidget;

impl Widget for &MainChatWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered()
            .title("#main-chat")
            .title_style(Style::new().bold().green())
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        block.render(area, buf);
    }
}
