mod widgets;

use crate::app::widgets::chat_list_sidebar::ChatListSidebarState;
use nebula_common::net::arti::ArtiConnector;
use nebula_common::{futures::Stream, tor_hsservice};
use ratatui::prelude::*;
use ratatui::widgets::ListState;
use ratatui::{DefaultTerminal, Frame};
use std::pin::Pin;
use std::sync::Arc;

const TICK_RATE: f64 = 30.0;

pub struct App {
    arti_connector: ArtiConnector,
    onion_service: Arc<tor_hsservice::RunningOnionService>,
    onion_service_stream: Pin<Box<dyn Stream<Item = tor_hsservice::RendRequest>>>,
    should_exit: bool,

    chat_list_sidebar_state: ChatListSidebarState,
}

impl App {
    pub fn new(
        should_exit: bool,
        onion_service_stream: Pin<Box<dyn Stream<Item = tor_hsservice::RendRequest>>>,
        onion_service: Arc<tor_hsservice::RunningOnionService>,
        arti_connector: ArtiConnector,
    ) -> Self {
        Self {
            arti_connector,
            onion_service,
            onion_service_stream,
            should_exit,
            chat_list_sidebar_state: ChatListSidebarState {
                is_selected: true,
                chat_list_state: ListState::default(),
            },
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let root_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(10),
                Constraint::Percentage(75),
                Constraint::Percentage(15),
            ]);

        let [left_area, center_area, right_area] = root_layout.areas(frame.area());

        let chat_list_sidebar = widgets::chat_list_sidebar::ChatListSidebarWidget;
        let connected_users_sidebar = widgets::connected_users_sidebar::ConnectedUsersSidebarWidget;
        let main_chat = widgets::main_chat::MainChatWidget;

        frame.render_stateful_widget(
            &chat_list_sidebar,
            left_area,
            &mut self.chat_list_sidebar_state,
        );
        frame.render_widget(&main_chat, center_area);
        frame.render_widget(&connected_users_sidebar, right_area);
    }
}
