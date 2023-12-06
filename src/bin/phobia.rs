
fn main() -> ! {
    if let Err(e) = phobia::logging::init() {
        eprintln!("Logging failed to init <{}>", e);
    }

    phobia::Application::<phobia::State>::run()
}

