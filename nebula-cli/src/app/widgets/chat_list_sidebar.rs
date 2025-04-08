use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, List, ListState, StatefulWidget};

#[derive(Debug, Clone)]
pub struct ChatListSidebarWidget;

#[derive(Clone)]
pub struct ChatListSidebarState {
    pub is_selected: bool,
    pub chat_list_state: ListState,
}

impl StatefulWidget for &ChatListSidebarWidget {
    type State = ChatListSidebarState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let main_block = Block::new()
            .title("Chat List")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(if state.is_selected {
                Style::new().green()
            } else {
                Style::new()
            });

        let chats = [
            "#main-chat",
            "#another-chat",
            "#yet-another-chat",
            "#very-long-chat-name-demo",
        ];
        let list = List::new(chats).block(main_block);

        list.render(area, buf, &mut state.chat_list_state);
    }
}
