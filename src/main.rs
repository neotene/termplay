use store::store::Store;
use termination::create_termination;
use ui::ui::UI;

mod store;
mod termination;
mod ui;

use termination::{ Interrupted, Terminator };

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (terminator, mut interrupt_receiver) = create_termination();
    let (store, state_receiver) = Store::new();
    let (ui, action_receiver) = UI::new();

    tokio::try_join!(
        store.do_loop(terminator, action_receiver, interrupt_receiver.resubscribe()),
        ui.do_loop(state_receiver, interrupt_receiver.resubscribe())
    )?;

    if let Ok(reason) = interrupt_receiver.recv().await {
        match reason {
            Interrupted::UserInt => println!("exited per user request"),
            Interrupted::OsSigInt => println!("exited because of an os sig int"),
        }
    } else {
        println!("exited because of an unexpected error");
    }

    Ok(())
}
