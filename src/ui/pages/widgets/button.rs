use ratatui::{ backend::Backend, layout::Rect, style::{ Color, Style }, widgets::Paragraph, Frame };
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    store::{ action::Action, state::State },
    ui::ui_object::ui_object::{ UIObject, UiRender },
};

pub struct Button {
    action_sender: UnboundedSender<Action>,
}

impl UIObject for Button {
    fn new(_state: &State, action_sender: UnboundedSender<Action>) -> Self {
        Self {
            action_sender,
        }
    }

    fn handle_key_event(&mut self, event: crossterm::event::Event) {
        match event {
            crossterm::event::Event::Key(key) => {
                match key.code {
                    crossterm::event::KeyCode::Enter => {
                        self.action_sender.send(Action::ShowRegister);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

pub struct RenderProperties {
    pub label: String,
    pub hovered: bool,
    pub area: Rect,
}

impl UiRender<RenderProperties> for Button {
    fn render<B: Backend>(&self, frame: &mut Frame<B>, properties: RenderProperties) {
        let style = if properties.hovered {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        frame.render_widget(Paragraph::new(properties.label.clone()).style(style), properties.area);
    }
}
