
use phobia::{
    application::Application,
    support::State,
};

fn main() {
    if let Err(e) = phobia::logging::init() {
        eprintln!("Logging failed to init <{}>", e);
    }

    let _ = State::<Application>::run_loop();
}

