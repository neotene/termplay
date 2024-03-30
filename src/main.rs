use core::net;

use store::{ action::{ self, Action }, state::State, store::Store };
use termination::create_termination;
use tokio::sync::mpsc;
use ui::ui::UI;

mod store;
mod termination;
mod ui;

use termination::Interrupted;

struct Network {
    action_sender: mpsc::UnboundedSender<Action>,
    state_sender: mpsc::UnboundedSender<State>,
    terminator: mpsc::UnboundedSender<Interrupted>,
}

impl Network {
    fn new(
        action_sender: mpsc::UnboundedSender<Action>,
        state_sender: mpsc::UnboundedSender<State>,
        terminator: mpsc::UnboundedSender<Interrupted>
    ) -> Self {
        Self { action_sender, state_sender, terminator }
    }

    async fn do_loop(self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[macro_use]
extern crate num_derive;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (terminator, mut interrupt_receiver) = create_termination();
    let (store, state_receiver) = Store::new();
    let (ui, action_receiver) = UI::new();
    let network = Network::new(action_receiver. , state_receiver, terminator.clone());
    tokio::try_join!(
        store.do_loop(terminator, action_receiver, interrupt_receiver.resubscribe()),
        ui.do_loop(state_receiver, interrupt_receiver.resubscribe())
        network.do_loop(state_receiver, interrupt_receiver.resubscribe()),
    )?;

    if let Ok(reason) = interrupt_receiver.recv().await {
        match reason {
            Interrupted::UserInt => println!("Goodbye!"),
            #[cfg(unix)]
            Interrupted::OsSigInt => println!("Interrupted."),
        }
    } else {
        println!("Unexpected termination.");
    }

    Ok(())
}
