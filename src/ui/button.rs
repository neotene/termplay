use ratatui::{
    backend::Backend,
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
}

pub struct InitProperties {
    pub label: String,
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
    // pub label: String,
    pub border_color: Color,
    pub area: Rect,
}

impl UIRender<RenderProperties> for Button {
    fn render<B: Backend>(&self, frame: &mut Frame<B>, properties: RenderProperties) {
        let style = Style::default().fg(properties.border_color);
        let paragraph = Paragraph::new(self.label.clone())
            .style(style)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, properties.area);
    }
}
