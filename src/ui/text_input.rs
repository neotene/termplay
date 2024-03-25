use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{ Color, Style },
    widgets::{ Block, Borders, Paragraph },
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{ store::{ action::Action, state::State }, ui::ui_object::{ UIObject, UIRender } };

pub struct TextInput {
    text: String,
    cursor_position: usize,
    cursor_limit: usize,
    is_password: bool,
}

pub struct InitProperties {
    pub cursor_limit: usize,
    pub is_password: bool,
}

impl UIObject<InitProperties> for TextInput {
    fn new(
        _state: &State,
        _action_sender: UnboundedSender<Action>,
        init_properties: InitProperties
    ) -> Self {
        Self {
            text: String::new(),
            cursor_position: 0,
            cursor_limit: init_properties.cursor_limit,
            is_password: init_properties.is_password,
        }
    }
    fn handle_key_event(&mut self, event: crossterm::event::Event) {
        match event {
            crossterm::event::Event::Key(key) => {
                match key.code {
                    crossterm::event::KeyCode::Backspace => {
                        self.delete_char();
                    }
                    crossterm::event::KeyCode::Left => {
                        self.move_cursor_left();
                    }
                    crossterm::event::KeyCode::Right => {
                        self.move_cursor_right();
                    }
                    crossterm::event::KeyCode::Char(to_insert) => {
                        self.enter_char(to_insert);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

impl TextInput {
    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        if self.text.len() >= self.cursor_limit {
            return;
        }

        self.text.insert(self.cursor_position, new_char);

        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.text.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.text.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.text = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.cursor_limit)
    }
}

pub struct RenderProperties {
    pub title: String,
    pub area: Rect,
    pub border_color: Color,
    pub show_cursor: bool,
}

impl UIRender<RenderProperties> for TextInput {
    fn render<B: Backend>(&self, frame: &mut Frame<B>, properties: RenderProperties) {
        let text_to_render: String;
        if self.is_password {
            text_to_render = "*".repeat(self.text.len());
        } else {
            text_to_render = self.text.clone();
        }
        let paragraph = Paragraph::new(text_to_render)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .title(properties.title)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(properties.border_color))
            );

        frame.render_widget(paragraph, properties.area);

        if properties.show_cursor {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            frame.set_cursor(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                properties.area.x + (self.cursor_position as u16) + 1,
                // Move one line down, from the border to the input line
                properties.area.y + 1
            )
        }
    }
}
