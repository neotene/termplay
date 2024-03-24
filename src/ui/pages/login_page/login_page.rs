use ratatui::backend::Backend;
use ratatui::layout::{ self, Constraint, Layout };
use ratatui::style::{ Color, Style };
use ratatui::widgets::{ Block, Borders };
use ratatui::Frame;
use tokio::sync::mpsc::UnboundedSender;

use crate::store::action::Action;
use crate::store::state::State;
use crate::ui::pages::widgets::text_input::{ self, TextInput };
use crate::ui::ui_object::ui_object::{ UiObject, UiRender };

pub struct LoginPage {
    login_field: TextInput,
}

impl UiObject for LoginPage {
    fn new(state: &State, action_sender: UnboundedSender<Action>) -> Self {
        Self {
            login_field: TextInput::new(state, action_sender),
        }
    }

    fn handle_key_event(&mut self, _event: crossterm::event::Event) {}
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
            area: modal_areas_vert_4[0],
            border_color: ratatui::style::Color::White,
        });
    }
}
