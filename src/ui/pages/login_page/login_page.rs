use ratatui::backend::Backend;
use ratatui::layout::{ self, Constraint, Direction, Layout };
use ratatui::style::{ Color, Style };
use ratatui::widgets::{ Block, Borders };
use ratatui::Frame;
use tokio::sync::mpsc::UnboundedSender;

use crate::store::action::Action;
use crate::store::state::State;
use crate::ui::pages::widgets::button::{ self, Button };
use crate::ui::pages::widgets::text_input::{ self, TextInput };
use crate::ui::ui_object::ui_object::{ UIObject, UiRender };

#[derive(Debug, Clone, PartialEq)]
pub enum Focus {
    LoginField,
    PasswordField,
    LoginButton,
    RegisterButton,
}

impl Focus {
    pub const COUNT: usize = 2;

    fn to_usize(&self) -> usize {
        match self {
            Focus::LoginField => 0,
            Focus::PasswordField => 1,
            Focus::LoginButton => 2,
            Focus::RegisterButton => 3,
        }
    }
}

impl TryFrom<usize> for Focus {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Focus::LoginField),
            1 => Ok(Focus::PasswordField),
            2 => Ok(Focus::LoginButton),
            3 => Ok(Focus::RegisterButton),
            _ => Err(()),
        }
    }
}

const DEFAULT_HOVERED_SECTION: Focus = Focus::LoginField;

pub struct LoginPage {
    login_field: TextInput,
    password_field: TextInput,
    login_button: Button,
    register_button: Button,
    last_hovered_section: Focus,
    active_section: Option<Focus>,
}

impl LoginPage {
    fn hover_next(&mut self) {
        let idx: usize = self.last_hovered_section.to_usize();
        let next_idx = (idx + 1) % Focus::COUNT;
        self.last_hovered_section = Focus::try_from(next_idx).unwrap();
    }

    fn hover_previous(&mut self) {
        let idx: usize = self.last_hovered_section.to_usize();
        let previous_idx = if idx == 0 { Focus::COUNT - 1 } else { idx - 1 };
        self.last_hovered_section = Focus::try_from(previous_idx).unwrap();
    }

    fn calculate_border_color(&self, section: Focus) -> Color {
        match (self.active_section.as_ref(), &self.last_hovered_section) {
            (Some(active_section), _) if active_section.eq(&section) => Color::Yellow,
            (_, last_hovered_section) if last_hovered_section.eq(&section) => Color::Blue,
            _ => Color::Reset,
        }
    }
}

impl UIObject for LoginPage {
    fn new(state: &State, action_sender: UnboundedSender<Action>) -> Self {
        Self {
            login_field: TextInput::new(state, action_sender.clone()),
            password_field: TextInput::new(state, action_sender.clone()),
            login_button: Button::new(state, action_sender.clone()),
            register_button: Button::new(state, action_sender.clone()),
            last_hovered_section: Focus::LoginField,
            active_section: None,
        }
    }

    fn handle_key_event(&mut self, event: crossterm::event::Event) {
        match event {
            crossterm::event::Event::Key(key) => {
                match key.code {
                    crossterm::event::KeyCode::Tab => {
                        self.hover_next();
                    }
                    crossterm::event::KeyCode::BackTab => {
                        self.hover_previous();
                    }
                    crossterm::event::KeyCode::Enter => {
                        match self.last_hovered_section {
                            Focus::LoginButton => {
                                self.login_button.handle_key_event(event);
                            }
                            Focus::RegisterButton => {
                                self.register_button.handle_key_event(event);
                            }
                            _ => {}
                        }
                    }
                    crossterm::event::KeyCode::Char(c) => {
                        match self.last_hovered_section {
                            Focus::LoginField => {
                                self.login_field.handle_key_event(event);
                            }
                            Focus::PasswordField => {
                                self.password_field.handle_key_event(event);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

impl UiRender<()> for LoginPage {
    fn render<B: Backend>(&self, frame: &mut Frame<B>, _properties: ()) {
        let areas_vert_3 = Layout::default()
            .direction(layout::Direction::Vertical)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(frame.size());

        let areas_center_3 = Layout::default()
            .direction(layout::Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(areas_vert_3[1]);

        let mut modal_area = areas_center_3[1];

        let modal_block = Block::default()
            .title("Welcome to Termplay!")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .style(Style::default().bg(Color::Black));

        // RENDER MODAL BLOCK
        frame.render_widget(modal_block, modal_area);

        modal_area.x += 2;
        modal_area.y += 2;
        modal_area.width -= 4;
        modal_area.height -= 4;

        let modal_areas_vert_4 = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
            ])
            .split(modal_area);

        let mut login_field_area = modal_areas_vert_4[0];
        login_field_area.height = 3;
        self.login_field.render(frame, text_input::RenderProperties {
            title: String::from("Login"),
            area: login_field_area,
            border_color: ratatui::style::Color::White,
        });

        let mut password_field_area = modal_areas_vert_4[1];
        password_field_area.height = 3;
        self.password_field.render(frame, text_input::RenderProperties {
            title: String::from("Password"),
            area: password_field_area,
            border_color: ratatui::style::Color::White,
        });

        let buttons_area = modal_areas_vert_4[3];
        let modal_buttons_areas_horiz_2 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(buttons_area);

        let mut login_button_area = modal_buttons_areas_horiz_2[0];
        login_button_area.height = 3;
        self.login_button.render(frame, button::RenderProperties {
            label: String::from("Login"),
            hovered: self.last_hovered_section.eq(&Focus::LoginButton),
            area: login_button_area,
        });

        let mut register_button_area = modal_buttons_areas_horiz_2[1];
        register_button_area.height = 3;
        self.register_button.render(frame, button::RenderProperties {
            label: String::from("Register"),
            hovered: self.last_hovered_section.eq(&Focus::RegisterButton),
            area: register_button_area,
        });
    }
}
