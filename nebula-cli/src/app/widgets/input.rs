use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::StatefulWidget;
use ratatui::style::Style;
use ratatui::widgets::BorderType::Rounded;
use ratatui::widgets::{Block, Paragraph, Widget};
use tui_input::Input;

#[derive(Clone)]
pub struct InputWidget {
    pub style: Style,
}

#[derive(Clone)]
pub struct InputWidgetState {
    pub input: Input,
}

impl StatefulWidget for &InputWidget {
    type State = InputWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let scroll = state.input.visual_scroll((area.width.max(3) - 3) as usize);

        let input_paragraph = Paragraph::new(state.input.value())
            .block(
                Block::bordered()
                    .title("Type your message:")
                    .border_type(Rounded)
                    .border_style(self.style),
            )
            .scroll((0, scroll as u16));

        input_paragraph.render(area, buf);
    }
}
