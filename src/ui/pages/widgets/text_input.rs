use ratatui::{ layout::Rect, style::{ Color, Style }, widgets::{ Block, Borders }, Frame };
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    store::{ action::Action, state::State },
    ui::ui_object::ui_object::{ UiObject, UiRender },
};

pub struct TextInput {
    text: String,
    cursor_position: usize,
}

impl UiObject for TextInput {
    fn new(_state: &State, _action_sender: UnboundedSender<Action>) -> Self {
        Self {
            text: String::new(),
            cursor_position: 0,
        }
    }
    fn handle_key_event(&mut self, event: crossterm::event::Event) {
        match event {
            crossterm::event::Event::Key(key) => {
                match key.code {
                    crossterm::event::KeyCode::Char(to_insert) => {
                        self.enter_char(to_insert);
                    }
                    crossterm::event::KeyCode::Backspace => {
                        self.delete_char();
                    }
                    crossterm::event::KeyCode::Left => {
                        self.move_cursor_left();
                    }
                    crossterm::event::KeyCode::Right => {
                        self.move_cursor_right();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

impl TextInput {
    fn enter_char(&self, to_insert: char) {
        let mut new_text = self.text.clone();
        new_text.insert(self.cursor_position, to_insert);
    }

    fn delete_char(&self) {
        if self.cursor_position > 0 {
            let mut new_text = self.text.clone();
            new_text.remove(self.cursor_position - 1);
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_position < self.text.len() {
            self.cursor_position += 1;
        }
    }
}

pub struct RenderProperties {
    pub title: String,
    pub area: Rect,
    pub border_color: Color,
}

impl UiRender<RenderProperties> for TextInput {
    fn render(&self, frame: &mut Frame, properties: RenderProperties) {
        let block = Block::default()
            .title(properties.title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(properties.border_color));
        frame.render_widget(&block, properties.area);
    }
}
