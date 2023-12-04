
use phobia::{
    application::Application,
    State,
};

fn main() -> ! {
    if let Err(e) = phobia::logging::init() {
        eprintln!("Logging failed to init <{}>", e);
    }

    phobia::prelude::register();

    State::<Application>::new().run_loop()
}

