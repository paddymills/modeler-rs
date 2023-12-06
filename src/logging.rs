
//! logging facilities

pub fn init() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}|{}]<{}> {}",
                chrono::Utc::now().format("%F %T"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("glium", log::LevelFilter::Off)
        .chain(std::io::stdout())
        .chain(fern::log_file("phobia.log")?)
        .apply()?;

    Ok(())
}