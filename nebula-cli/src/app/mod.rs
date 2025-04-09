mod panes;
mod widgets;

use crate::app::panes::center::CenterPaneState;
use crate::app::widgets::input::InputWidgetState;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers};
use nebula_common::futures::{FutureExt, StreamExt};
use nebula_common::net::arti::{ArtiConnector, TorTriggerEvent};
use ratatui::prelude::*;
use ratatui::widgets::BorderType::Rounded;
use ratatui::widgets::{Block, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use std::cmp::PartialEq;
use std::sync::Arc;
use tokio::sync::oneshot::error::TryRecvError;
use tokio::sync::oneshot::Receiver;
use tui_input::{Input, InputRequest};

pub struct App {
    should_exit: bool,
    input_mode: InputMode,
    ct_event_stream: EventStream,
    center_pane_state: CenterPaneState,

    arti_connector: Arc<ArtiConnector>,
    arti_status_rx: Receiver<TorTriggerEvent>,
    is_arti_started: bool,
    is_arti_failed: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Insert,
    Command,
}

impl App {
    pub fn new(arti_connector: Arc<ArtiConnector>, arti_status_rx: Receiver<TorTriggerEvent>) -> Self {
        let input_mode = InputMode::Normal;

        let center_pane_state = CenterPaneState {
            input_widget_state: InputWidgetState {
                input: Input::default(),
            },
            mode: input_mode,
            messages: vec![],
        };

        Self {
            should_exit: false,
            input_mode,
            ct_event_stream: EventStream::new(),
            center_pane_state,
            arti_connector,
            arti_status_rx,
            is_arti_started: false,
            is_arti_failed: false,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events().await?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        if !self.is_arti_started && !self.is_arti_failed {
            match self.arti_status_rx.try_recv() {
                Ok(event) => match event {
                    TorTriggerEvent::Running => {
                        self.is_arti_started = true;
                    }
                    TorTriggerEvent::Failed => {
                        self.is_arti_failed = true;
                    }
                },
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Closed) => {
                    self.is_arti_failed = true;
                }
            }
        }

        if self.is_arti_failed {
            let style = Style::new().red();

            let failure_message = Paragraph::new("Startup failed!\nPress C-q to exit.")
                .block(Block::bordered().border_type(Rounded).border_style(style))
                .style(style)
                .alignment(Alignment::Center)
                .centered();

            frame.render_widget(failure_message, frame.area());
        } else if self.is_arti_started {
            let root_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(10),
                    Constraint::Percentage(75),
                    Constraint::Percentage(15),
                ]);

            let [_left_area, center_area, _right_area] = root_layout.areas(frame.area());

            panes::center::render(frame, center_area, &mut self.center_pane_state);
            let root_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(10),
                    Constraint::Percentage(75),
                    Constraint::Percentage(15),
                ]);

            let [_left_area, center_area, _right_area] = root_layout.areas(frame.area());

            panes::center::render(frame, center_area, &mut self.center_pane_state);
        } else {
            let style = Style::new().blue();
            let message = Paragraph::new("The application is starting...")
                .block(Block::bordered().border_type(Rounded).border_style(style))
                .style(style)
                .centered();

            frame.render_widget(message, frame.area());
        }
    }

    async fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        tokio::select! {
            event = self.ct_event_stream.next().fuse() => {
                if let Some(Ok(Event::Key(key))) = event {
                    if key.is_press() {
                        self.on_key_event(key).await;
                    }
                }
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {},
        }

        Ok(())
    }

    async fn on_key_event(&mut self, event: KeyEvent) {
        match (self.input_mode, event.modifiers, event.code) {
            (InputMode::Normal, KeyModifiers::CONTROL, KeyCode::Char('q') | KeyCode::Char('Q')) => {
                self.should_exit = true;
            }
            (InputMode::Normal, _, KeyCode::Char('i') | KeyCode::Char('I')) => {
                self.input_mode = InputMode::Insert;
                self.center_pane_state.mode = InputMode::Insert;
            }
            (InputMode::Normal, _, KeyCode::Char('c')) => {
                self.input_mode = InputMode::Command;
            }
            (InputMode::Insert, _, KeyCode::Esc) => {
                self.input_mode = InputMode::Normal;
                self.center_pane_state.mode = InputMode::Normal;
            }
            (InputMode::Insert, _, KeyCode::Char(c)) => {
                let input_request = InputRequest::InsertChar(c);
                self.center_pane_state
                    .input_widget_state
                    .input
                    .handle(input_request);
            }
            (InputMode::Insert, _, KeyCode::Backspace) => {
                let input_request = InputRequest::DeletePrevChar;
                self.center_pane_state
                    .input_widget_state
                    .input
                    .handle(input_request);
            }
            (InputMode::Insert, _, KeyCode::Enter) => {
                if self
                    .center_pane_state
                    .input_widget_state
                    .input
                    .value()
                    .is_empty()
                {
                    return;
                }

                self.center_pane_state.messages.push(
                    self.center_pane_state
                        .input_widget_state
                        .input
                        .value()
                        .to_string(),
                );

                self.center_pane_state.input_widget_state.input.reset();
            }
            (InputMode::Command, _, KeyCode::Esc) => {
                self.input_mode = InputMode::Normal;
            }
            _ => {}
        }
    }
}
