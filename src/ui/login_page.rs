use crossterm::event::KeyEventKind;
use ratatui::backend::Backend;
use ratatui::layout::{ self, Alignment, Constraint, Direction, Layout };
use ratatui::style::{ Color, Style };
use ratatui::widgets::{ Block, Borders };
use ratatui::Frame;
use tokio::sync::mpsc::UnboundedSender;

use crate::store::action::Action;
use crate::store::state::State;
use crate::ui::button::{ self, Button };
use crate::ui::text_input::{ self, TextInput };
use crate::ui::ui_object::{ UIObject, UIRender };

use super::utils;

#[derive(FromPrimitive, ToPrimitive, Eq, PartialEq, Clone, Copy)]
pub enum Focus {
    LoginField,
    PasswordField,
    LoginButton,
    RegisterButton,
    ExitButton,
}

const DEFAULT_HOVERED_SECTION: Focus = Focus::LoginField;

pub struct LoginPage {
    login_field: TextInput,
    password_field: TextInput,
    login_button: Button,
    register_button: Button,
    exit_button: Button,
    last_hovered_section: Focus,
    active_section: Option<Focus>,
}

impl LoginPage {
    fn calculate_show_cursor(&self, focus: Focus) -> bool {
        match (self.active_section.as_ref(), &self.last_hovered_section) {
            (Some(active_section), _) if active_section.eq(&focus) => false,
            (_, last_hovered_section) if last_hovered_section.eq(&focus) => true,
            _ => false,
        }
    }
}

impl UIObject<()> for LoginPage {
    fn new(state: &State, action_sender: UnboundedSender<Action>, _: ()) -> Self {
        Self {
            login_field: TextInput::new(state, action_sender.clone(), text_input::InitProperties {
                cursor_limit: 40,
                is_password: false,
            }),
            password_field: TextInput::new(
                state,
                action_sender.clone(),
                text_input::InitProperties {
                    cursor_limit: 40,
                    is_password: true,
                }
            ),
            login_button: Button::new(state, action_sender.clone(), button::InitProperties {
                label: String::from("Login"),
                action_to_send: Action::Login,
            }),
            register_button: Button::new(state, action_sender.clone(), button::InitProperties {
                label: String::from("Register"),
                action_to_send: Action::ShowRegister,
            }),
            exit_button: Button::new(state, action_sender.clone(), button::InitProperties {
                label: String::from("Exit"),
                action_to_send: Action::Exit,
            }),
            last_hovered_section: DEFAULT_HOVERED_SECTION,
            active_section: None,
        }
    }

    fn move_with_state(self, state: &State) -> Self {
        Self {
            login_field: self.login_field.move_with_state(state),
            password_field: self.password_field.move_with_state(state),
            login_button: self.login_button.move_with_state(state),
            register_button: self.register_button.move_with_state(state),
            exit_button: self.exit_button.move_with_state(state),
            last_hovered_section: self.last_hovered_section,
            active_section: self.active_section,
        }
    }

    fn handle_key_event(&mut self, event: crossterm::event::Event) {
        match event {
            crossterm::event::Event::Key(key) => {
                if key.kind != KeyEventKind::Press {
                    return;
                }
                match key.code {
                    crossterm::event::KeyCode::Tab => {
                        self.last_hovered_section = utils::cycle(
                            Focus::LoginField,
                            Focus::ExitButton,
                            self.last_hovered_section,
                            1
                        );
                    }
                    crossterm::event::KeyCode::BackTab => {
                        self.last_hovered_section = utils::cycle(
                            Focus::LoginField,
                            Focus::ExitButton,
                            self.last_hovered_section,
                            -1
                        );
                    }
                    _ => {
                        let active_section = self.active_section
                            .as_ref()
                            .unwrap_or(&self.last_hovered_section);
                        match active_section {
                            Focus::LoginField => {
                                self.login_field.handle_key_event(event);
                            }
                            Focus::PasswordField => {
                                self.password_field.handle_key_event(event);
                            }
                            Focus::LoginButton => {
                                self.login_button.handle_key_event(event);
                            }
                            Focus::RegisterButton => {
                                self.register_button.handle_key_event(event);
                            }
                            Focus::ExitButton => {
                                self.exit_button.handle_key_event(event);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

impl UIRender<()> for LoginPage {
    fn render<B: Backend>(&self, frame: &mut Frame<B>, _properties: ()) {
        let page_block = Block::default()
            .title("Termplay")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .style(Style::default().bg(Color::Black));

        // RENDER PAGE BLOCK
        frame.render_widget(page_block, frame.size());

        let areas_vert_3 = Layout::default()
            .direction(layout::Direction::Vertical)
            .constraints([
                Constraint::Percentage(35),
                Constraint::Percentage(30),
                Constraint::Percentage(35),
            ])
            .split(frame.size());

        let areas_center_3 = Layout::default()
            .direction(layout::Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(areas_vert_3[1]);

        let mut modal_area = areas_center_3[1];

        let modal_block = Block::default()
            .title("Welcome to Termplay!")
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

        let modal_areas_vert_5 = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Min(3),
                Constraint::Percentage(100),
                Constraint::Min(3),
                Constraint::Min(3),
            ])
            .split(modal_area);

        let mut login_field_area = modal_areas_vert_5[0];
        login_field_area.height = 3;
        // RENDER LOGIN FIELD
        self.login_field.render(frame, text_input::RenderProperties {
            title: String::from("Login"),
            area: login_field_area,
            border_color: utils::calculate_border_color(
                self.active_section,
                self.last_hovered_section,
                Focus::LoginField
            ),
            show_cursor: self.calculate_show_cursor(Focus::LoginField),
        });

        let mut password_field_area = modal_areas_vert_5[1];
        password_field_area.height = 3;
        // RENDER PASSWORD FIELD
        self.password_field.render(frame, text_input::RenderProperties {
            title: String::from("Password"),
            area: password_field_area,
            border_color: utils::calculate_border_color(
                self.active_section,
                self.last_hovered_section,
                Focus::PasswordField
            ),
            show_cursor: self.calculate_show_cursor(Focus::PasswordField),
        });

        let buttons_area = modal_areas_vert_5[3];
        let modal_buttons_areas_horiz_3 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(15), Constraint::Percentage(100), Constraint::Min(15)])
            .split(buttons_area);

        let mut login_button_area = modal_buttons_areas_horiz_3[0];
        login_button_area.height = 3;
        // RENDER LOGIN BUTTON
        self.login_button.render(frame, button::RenderProperties {
            // label: String::from("Login"),
            border_color: utils::calculate_border_color(
                self.active_section,
                self.last_hovered_section,
                Focus::LoginButton
            ),
            area: login_button_area,
        });

        let mut register_button_area = modal_buttons_areas_horiz_3[2];
        register_button_area.height = 3;
        // RENDER REGISTER BUTTON
        self.register_button.render(frame, button::RenderProperties {
            // label: String::from("Register"),
            border_color: utils::calculate_border_color(
                self.active_section,
                self.last_hovered_section,
                Focus::RegisterButton
            ),
            area: register_button_area,
        });

        let mut exit_button_area = modal_areas_vert_5[4];
        exit_button_area.height = 3;
        // RENDER EXIT BUTTON
        self.exit_button.render(frame, button::RenderProperties {
            // label: String::from("Exit"),
            border_color: utils::calculate_border_color(
                self.active_section,
                self.last_hovered_section,
                Focus::ExitButton
            ),
            area: exit_button_area,
        });
    }
}
