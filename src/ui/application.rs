use ratatui::{
    layout::{ Alignment, Layout },
    style::{ Color, Style },
    widgets::{ Block, Borders, Paragraph },
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::store::{ action::Action, state::State };

use super::{
    exit_modal::ExitModal,
    login_page::LoginPage,
    register_page::RegisterPage,
    ui_object::{ UIObject, UIRender },
};

enum ActivePage {
    LoginPage,
    RegisterPage,
}

pub struct Application {
    login_page: LoginPage,
    register_page: RegisterPage,
    exit_modal: ExitModal,
    active_page: ActivePage,
    action_sender: UnboundedSender<Action>,
    show_exit_modal: bool,
    error_message: String,
}

impl UIObject<()> for Application {
    fn new(state: &State, action_sender: UnboundedSender<Action>, _: ()) -> Self {
        Self {
            login_page: LoginPage::new(state, action_sender.clone(), ()),
            register_page: RegisterPage::new(state, action_sender.clone(), ()),
            exit_modal: ExitModal::new(state, action_sender.clone(), ()),
            active_page: ActivePage::LoginPage,
            action_sender,
            show_exit_modal: false,
            error_message: String::new(),
        }
    }

    fn move_with_state(self, state: &State) -> Self {
        Self {
            login_page: self.login_page.move_with_state(state),
            register_page: self.register_page.move_with_state(state),
            active_page: match state.is_registering {
                false => ActivePage::LoginPage,
                true => ActivePage::RegisterPage,
            },
            action_sender: self.action_sender,
            exit_modal: self.exit_modal.move_with_state(state),
            show_exit_modal: state.show_exit_confirmation,
            error_message: state.error_message.clone(),
        }
    }

    fn handle_key_event(&mut self, event: crossterm::event::Event) {
        match event {
            crossterm::event::Event::Key(key) => {
                if key.code == crossterm::event::KeyCode::Esc {
                    self.action_sender.send(Action::PreExit).unwrap();
                }
            }
            _ => {}
        }

        if self.show_exit_modal {
            self.exit_modal.handle_key_event(event);
            return;
        }

        match self.active_page {
            ActivePage::LoginPage => self.login_page.handle_key_event(event),
            ActivePage::RegisterPage => self.register_page.handle_key_event(event),
        }
    }
}

impl UIRender<()> for Application {
    fn render(&self, frame: &mut Frame, properties: ()) {
        // PAGE BLOCK
        let page_block = Block::default()
            .title("Termplay")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .style(Style::default().bg(Color::Black));

        frame.render_widget(page_block, frame.size());

        // ERROR MESSAGE
        let error_message = Paragraph::new(self.error_message.as_str())
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center);

        let areas_for_error_message = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(
                [
                    ratatui::layout::Constraint::Percentage(10),
                    ratatui::layout::Constraint::Percentage(10),
                    ratatui::layout::Constraint::Percentage(10),
                    ratatui::layout::Constraint::Percentage(10),
                    ratatui::layout::Constraint::Percentage(10),
                    ratatui::layout::Constraint::Percentage(10),
                    ratatui::layout::Constraint::Percentage(10),
                    ratatui::layout::Constraint::Percentage(10),
                    ratatui::layout::Constraint::Percentage(10),
                    ratatui::layout::Constraint::Percentage(10),
                ].as_ref()
            )
            .split(frame.size());

        let mut error_message_area = areas_for_error_message[9];
        error_message_area.width -= 2;
        error_message_area.height -= 2;
        error_message_area.x += 1;
        frame.render_widget(error_message, error_message_area);

        // CURRENT PAGE
        match self.active_page {
            ActivePage::LoginPage => self.login_page.render(frame, properties),
            ActivePage::RegisterPage => self.register_page.render(frame, properties),
        }

        // EXIT MODAL
        if self.show_exit_modal {
            self.exit_modal.render(frame, properties);
        }
    }
}
