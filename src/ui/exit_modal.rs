use crossterm::event::KeyEventKind;
use num_derive::FromPrimitive;
use ratatui::{
    layout::{ Alignment, Constraint, Direction, Layout },
    style::{ Color, Style },
    widgets::{ Block, Borders, Clear },
};
use tokio::sync::mpsc::UnboundedSender;

use crate::store::{ action::Action, state::State };

use super::{ button::{ self, Button }, ui_object::{ UIObject, UIRender }, utils };

#[derive(FromPrimitive, ToPrimitive, Eq, PartialEq, Clone, Copy)]
pub enum Focus {
    YesButton,
    NoButton,
}

const DEFAULT_HOVERED_SECTION: Focus = Focus::NoButton;

pub struct ExitModal {
    label: String,
    action_sender: UnboundedSender<Action>,
    yes_button: Button,
    no_button: Button,
    last_hovered_section: Focus,
    active_section: Option<Focus>,
}

impl UIObject<()> for ExitModal {
    fn new(_state: &State, action_sender: UnboundedSender<Action>, _init_properties: ()) -> Self {
        Self {
            label: "Are you sure you want to exit?".to_string(),
            action_sender: action_sender.clone(),
            yes_button: Button::new(_state, action_sender.clone(), button::InitProperties {
                label: "Yes".to_string(),
                action_to_send: Action::Exit,
            }),
            no_button: Button::new(_state, action_sender.clone(), button::InitProperties {
                label: "No".to_string(),
                action_to_send: Action::CancelExit,
            }),
            last_hovered_section: DEFAULT_HOVERED_SECTION,
            active_section: None,
        }
    }

    fn move_with_state(self, _state: &State) -> Self {
        self
    }

    fn handle_key_event(&mut self, event: crossterm::event::Event) {
        match event {
            crossterm::event::Event::Key(key) => {
                if key.kind != KeyEventKind::Press {
                    return;
                }
                match key.code {
                    crossterm::event::KeyCode::Esc => {
                        self.action_sender.send(Action::CancelExit).unwrap();
                    }
                    crossterm::event::KeyCode::Tab => {
                        self.last_hovered_section = utils::cycle(
                            Focus::YesButton,
                            Focus::NoButton,
                            self.last_hovered_section,
                            1
                        );
                    }
                    crossterm::event::KeyCode::BackTab => {
                        self.last_hovered_section = utils::cycle(
                            Focus::YesButton,
                            Focus::NoButton,
                            self.last_hovered_section,
                            -1
                        );
                    }
                    _ => {
                        let active_section = self.active_section
                            .as_ref()
                            .unwrap_or(&self.last_hovered_section);
                        match active_section {
                            Focus::YesButton => {
                                self.yes_button.handle_key_event(event);
                            }
                            Focus::NoButton => {
                                self.no_button.handle_key_event(event);
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

impl UIRender<()> for ExitModal {
    fn render<B: ratatui::prelude::Backend>(
        &self,
        frame: &mut ratatui::prelude::Frame<B>,
        _properties: ()
    ) {
        let areas_vert_3 = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(44),
                Constraint::Percentage(12),
                Constraint::Percentage(44),
            ])
            .split(frame.size());

        let areas_center_3 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .split(areas_vert_3[1]);

        let mut modal_area = areas_center_3[1];

        frame.render_widget(Clear::default(), modal_area);

        let modal_block = Block::default()
            .title(self.label.to_string())
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .style(Style::default().bg(Color::Black));

        // RENDER MODAL BLOCK
        frame.render_widget(modal_block, modal_area);

        modal_area.x += 4;
        modal_area.y += 2;
        modal_area.width -= 8;
        modal_area.height -= 4;

        let button_areas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(modal_area);

        let yes_button_area = button_areas[0];
        let no_button_area = button_areas[1];

        let yes_button_border_color = utils::calculate_border_color(
            self.active_section,
            self.last_hovered_section,
            Focus::YesButton
        );
        let no_button_border_color = utils::calculate_border_color(
            self.active_section,
            self.last_hovered_section,
            Focus::NoButton
        );

        self.yes_button.render(frame, button::RenderProperties {
            border_color: yes_button_border_color,
            area: yes_button_area,
        });

        self.no_button.render(frame, button::RenderProperties {
            border_color: no_button_border_color,
            area: no_button_area,
        });
    }
}
