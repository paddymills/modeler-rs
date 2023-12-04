
use phobia::{
    application::Application,
    support::State,
};

fn main() {
    if let Err(e) = phobia::logging::init() {
        eprintln!("Logging failed to init <{}>", e);
    }

    phobia::prelude::register();

    let _ = State::<Application>::run_loop();
}

