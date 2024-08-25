pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message.to_string()
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())

        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

