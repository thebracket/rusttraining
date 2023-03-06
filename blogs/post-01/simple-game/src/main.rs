mod state;
use bracket_lib::terminal::{main_loop, BTermBuilder};
use state::State;

fn main() {
    let result = BTermBuilder::simple80x50()
        .with_title("Hello Bracket World")
        .build();

    match result {
        Ok(bterm) => {
            let _ = main_loop(bterm, State::new());
        }
        Err(err) => {
            println!("An error has occurred: {err:?}");
            println!("Aborting");
        }
    }
}
