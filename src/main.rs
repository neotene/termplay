use crossterm::{
    event::{ self, KeyCode, KeyEvent, KeyEventKind },
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
    ExecutableCommand,
};
use ratatui::{ layout::Constraint, prelude::{ CrosstermBackend, Terminal } };
use utils::centered_rect;
use std::io::{ stdout, Result };

mod ui;
mod utils;

use ui::{ field::Field, layout::Layout, line::Line, ui::UI, widget_holder::WidgetHolder };
use ui::button::Button;
use ui::label::Label;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let login_email_field = Field::new(
        String::from("Email"),
        String::from("Please enter your email address"),
        false
    );

    let login_password_field = Field::new(
        String::from("Password"),
        String::from("Please enter your password"),
        true
    );

    let login_button = Button::new(String::from("Login"), || {});

    // register modale
    let register_email_field = Field::new(
        String::from("Email"),
        String::from("Please enter your email address"),
        false
    );

    let register_email_confirm_field = Field::new(
        String::from("Confirm Email"),
        String::from("Please confirm your email address"),
        false
    );

    let register_password_field = Field::new(
        String::from("Password"),
        String::from("Please enter your password"),
        true
    );

    let register_password_confirm_field = Field::new(
        String::from("Confirm Password"),
        String::from("Please confirm your password"),
        true
    );

    let register_back_button = Button::new(String::from("Back"), || {});

    let register_register_button = Button::new(String::from("Register"), || {});

    let _register_layout = Layout::new(
        vec![
            Line::new(
                vec![WidgetHolder::new(Box::new(register_email_field))],
                vec![Constraint::Percentage(100)]
            ),
            Line::new(
                vec![WidgetHolder::new(Box::new(register_email_confirm_field))],
                vec![Constraint::Percentage(100)]
            ),
            Line::new(
                vec![WidgetHolder::new(Box::new(register_password_field))],
                vec![Constraint::Percentage(100)]
            ),
            Line::new(
                vec![WidgetHolder::new(Box::new(register_password_confirm_field))],
                vec![Constraint::Percentage(100)]
            ),
            Line::new(
                vec![
                    WidgetHolder::new(Box::new(register_back_button)),
                    WidgetHolder::new(Box::new(register_register_button))
                ],
                vec![Constraint::Percentage(50), Constraint::Percentage(50)]
            )
        ],
        vec![
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3)
        ],
        centered_rect(terminal.size()?, 50, 25)
    );
    // end register modale

    let register_button = Button::new(
        String::from("Register"),
        || {
            // ui.push_layout(register_layout);
        }
    );

    let fake_label = Label::default();

    let mut ui = UI::new(
        vec![
            Layout::new(
                vec![
                    Line::new(
                        vec![WidgetHolder::new(Box::new(login_email_field))],
                        vec![Constraint::Percentage(100)]
                    ),
                    Line::new(
                        vec![WidgetHolder::new(Box::new(login_password_field))],
                        vec![Constraint::Percentage(100)]
                    ),
                    Line::new(
                        vec![WidgetHolder::new(Box::new(fake_label))],
                        vec![Constraint::Percentage(100)]
                    ),
                    Line::new(
                        vec![
                            WidgetHolder::new(Box::new(login_button)),
                            WidgetHolder::new(Box::new(register_button))
                        ],
                        vec![Constraint::Percentage(50), Constraint::Percentage(50)]
                    )
                ],
                vec![
                    Constraint::Min(3),
                    Constraint::Min(3),
                    Constraint::Percentage(100),
                    Constraint::Min(3)
                ],
                centered_rect(terminal.size()?, 50, 25)
            )
        ]
    );
    loop {
        let mut key_event = KeyEvent::from(KeyCode::Null);
        if event::poll(std::time::Duration::from_millis(16))? {
            let event = event::read()?;
            match event {
                event::Event::Key(key) => {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Esc {
                        break;
                    }
                    key_event = key;
                }
                _ => (),
            }
        }

        if key_event.code != KeyCode::Null {
            ui.update(key_event);
        }

        terminal.draw(|frame| {
            ui.draw(frame);
        })?;
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
