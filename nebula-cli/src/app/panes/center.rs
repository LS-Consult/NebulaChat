use crate::app::widgets::input::{InputWidget, InputWidgetState};
use crate::app::InputMode;
use ratatui::layout::Alignment::Center;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use ratatui::Frame;

#[derive(Clone)]
pub struct CenterPaneState {
    pub node_address: Option<String>,
    pub input_widget_state: InputWidgetState,
    pub mode: InputMode,
    pub messages: Vec<String>,
}

pub fn render(frame: &mut Frame, area: Rect, state: &mut CenterPaneState) {
    let [history_area, input_area] = Layout::default()
        .constraints(vec![Constraint::Fill(1), Constraint::Max(3)])
        .areas(area);

    // History Area
    render_message_history(frame, history_area, &state.messages, state.node_address.clone());

    // Input Area
    let input_widget = InputWidget {
        style: match state.mode {
            InputMode::Insert => Style::new().green(),
            _ => Style::default(),
        },
    };

    let width = area.width.max(3) - 3;
    let scroll = state.input_widget_state.input.visual_scroll(width as usize);
    if state.mode == InputMode::Insert {
        let x = state.input_widget_state.input.visual_cursor().max(scroll) - scroll + 1;
        frame.set_cursor_position((input_area.x + x as u16, input_area.y + 1));
    }

    frame.render_stateful_widget(&input_widget, input_area, &mut state.input_widget_state);
}

fn render_message_history(
    frame: &mut Frame,
    area: Rect,
    messages: &[String],
    node_address: Option<String>,
) {
    // Create a block for the history area
    let history_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title_bottom(node_address.unwrap_or(String::from("")))
        .title_alignment(Center);

    // Create an area inside the block to render messages
    let inner_area = history_block.inner(area);

    // Calculate which messages can fit in the available space
    let available_height = inner_area.height as usize;

    // Start from the most recent messages and calculate backward
    let mut visible_messages = Vec::new();
    let mut lines_used = 0;

    for message in messages.iter().rev() {
        // Parse message to extract username (assuming format like "username: content")
        let (username, content) = if let Some(idx) = message.find(':') {
            let (username, content) = message.split_at(idx + 1);
            (username.trim(), content.trim())
        } else {
            ("shimori:", message.as_str()) // Default if no username found
        };

        // Calculate how many lines this message will take
        // Username line + content lines + empty line separator
        let content_line_count =
            (content.len() as u16).saturating_add(inner_area.width - 1) / inner_area.width;
        let content_height = content_line_count.max(1) as usize;

        // Each message now takes: 1 (username) + content_height + 1 (separator)
        let lines_needed = 1 + content_height + 1;

        // If we can fit this message, add it
        if lines_used + lines_needed <= available_height {
            visible_messages.push((username, content));
            lines_used += lines_needed;
        } else {
            // We can't fit any more messages
            break;
        }
    }

    // Reverse back to chronological order (oldest first)
    visible_messages.reverse();

    // Render each message as a paragraph
    let mut current_y = 0;
    for (username, content) in visible_messages {
        // Check if rendering the next message would exceed the available height
        if current_y >= inner_area.height {
            break;
        }

        // Create a paragraph for the username with blue and bold styling
        let username_paragraph =
            Paragraph::new(username.to_string()).style(Style::default().blue().bold());

        // Render the username paragraph
        let username_area = Rect {
            x: inner_area.x,
            y: inner_area.y + current_y,
            width: inner_area.width,
            height: 1,
        };
        frame.render_widget(username_paragraph, username_area);
        current_y += 1;

        // Create a paragraph for the message content
        let content_paragraph = Paragraph::new(content.to_string()).wrap(Wrap { trim: true });

        // Calculate the height this content will take
        let content_line_count =
            (content.len() as u16).saturating_add(inner_area.width - 1) / inner_area.width;
        let content_height = content_line_count.max(1);

        // Render the content paragraph
        let content_area = Rect {
            x: inner_area.x,
            y: inner_area.y + current_y,
            width: inner_area.width,
            height: content_height,
        };
        frame.render_widget(content_paragraph, content_area);
        current_y += content_height;

        // Add an empty line as a separator (increment Y position)
        current_y += 1;
    }

    // Render the block around all messages
    frame.render_widget(history_block, area);
}
