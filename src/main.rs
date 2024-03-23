use std::io::stdout;

use crossterm::{
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
    ExecutableCommand,
};
use ratatui::{ layout::Constraint, prelude::{ CrosstermBackend, Terminal } };
use utils::centered_rect;

mod ui;
mod utils;

use ui::{ field::Field, layout::Layout, line::Line, ui::UI, widget_holder::WidgetHolder };
use ui::button::Button;
use ui::label::Label;

use tokio::net::TcpListener;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let term_size = terminal.size()?;
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

    let login_button = Button::new(
        String::from("Login"),
        Box::new(|_layout, _layouts| Ok(true))
    );

    let register_button = Button::new(
        String::from("Register"),
        Box::new(
            move |_layout, layouts| -> Result<bool, std::io::Error> {
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

                let register_back_button = Button::new(
                    String::from("Back"),
                    Box::new(|_layout, layouts| {
                        layouts.pop();
                        Ok(true)
                    })
                );

                let register_register_button = Button::new(
                    String::from("Register"),
                    Box::new(|_layout, _layoutss| Ok(true))
                );

                let register_layout = Layout::new(
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
                    centered_rect(term_size, 50, 25)
                );
                layouts.push(register_layout);
                Ok(true)
            }
        )
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
        if !ui.update() {
            break;
        }

        terminal.draw(|frame| {
            ui.draw(frame);
        })?;
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
