use std::io::{ self, Stdout };

use anyhow::Context;
use crossterm::{
    event::{ DisableMouseCapture, EnableMouseCapture, EventStream },
    execute,
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
};
use ratatui::{ backend::CrosstermBackend, Terminal };
use tokio::sync::{ broadcast, mpsc::{ self, UnboundedReceiver } };
use tokio_stream::StreamExt;
use crate::{ store::action::Action, termination::Interrupted, ui::ui_object::ui_object::UiRender };

use super::{ application::Application, ui_object::ui_object::UIObject };

pub struct UI {
    action_sender: mpsc::UnboundedSender<Action>,
}

impl UI {
    pub fn new() -> (Self, UnboundedReceiver<Action>) {
        let (action_sender, action_receiver) = mpsc::unbounded_channel();

        (Self { action_sender }, action_receiver)
    }
    pub async fn do_loop(
        &self,
        mut _state_receiver: UnboundedReceiver<crate::store::state::State>,
        mut _interrupt_receiver: broadcast::Receiver<Interrupted>
    ) -> anyhow::Result<Interrupted> {
        let mut application = {
            let state = _state_receiver.recv().await.unwrap();

            Application::new(&state, self.action_sender.clone())
        };

        let mut terminal = setup_terminal()?;
        let mut crossterm_events = EventStream::new();

        let _result: anyhow::Result<Interrupted> = loop {
            tokio::select! {
                Some(maybe_event) = crossterm_events.next() => {
                    application.handle_key_event(maybe_event.unwrap());
                },
                // Handle state updates
                Some(_state) = _state_receiver.recv() => {
                    // app_router = app_router.move_with_state(&state);
                },
                // Catch and handle interrupt signal to gracefully shutdown
                Ok(interrupted) = _interrupt_receiver.recv() => {
                    break Ok(interrupted);
                }
            }

            if
                let Err(err) = terminal
                    .draw(|frame| application.render(frame, ()))
                    .context("could not render to the terminal")
            {
                break Err(err);
            }
        };

        restore_terminal(&mut terminal)?;

        Ok(Interrupted::UserInt)
    }
}

fn setup_terminal() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();

    enable_raw_mode()?;

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<()> {
    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(terminal.show_cursor()?)
}
