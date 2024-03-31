use ratatui::{
    layout::{ Alignment, Rect },
    style::{ Color, Style },
    widgets::{ Block, Borders, Paragraph },
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{ store::{ action::Action, state::State }, ui::ui_object::{ UIObject, UIRender } };

pub struct Button {
    action_sender: UnboundedSender<Action>,
    pub label: String,
    pub action_to_send: Action,
}

pub struct InitProperties {
    pub label: String,
    pub action_to_send: Action,
}

impl UIObject<InitProperties> for Button {
    fn new(
        _state: &State,
        action_sender: UnboundedSender<Action>,
        init_properties: InitProperties
    ) -> Self {
        Self {
            action_sender,
            label: init_properties.label,
            action_to_send: init_properties.action_to_send,
        }
    }

    fn move_with_state(self, _state: &State) -> Self {
        self
    }

    fn handle_key_event(&mut self, event: crossterm::event::Event) {
        match event {
            crossterm::event::Event::Key(key) => {
                match key.code {
                    crossterm::event::KeyCode::Enter => {
                        if let Err(err) = self.action_sender.send(self.action_to_send.clone()) {
                            eprintln!("Failed to send action: {:?}", err);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

pub struct RenderProperties {
    // pub label: String,
    pub border_color: Color,
    pub area: Rect,
}

impl UIRender<RenderProperties> for Button {
    fn render(&self, frame: &mut Frame, properties: RenderProperties) {
        let style = Style::default().fg(properties.border_color);
        let paragraph = Paragraph::new(self.label.clone())
            .style(style)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, properties.area);
    }
}
