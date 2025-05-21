// api/src/utilities/logger.rs
// Logger setup

pub fn setup_logger() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{color}{level:<5}{reset} [{}] {}",
                chrono::Local::now().format("%H:%M:%S"),
                message,
                color = match record.level() {
                    log::Level::Error => "\x1B[31m",
                    log::Level::Warn => "\x1B[33m",
                    log::Level::Info => "\x1B[32m",
                    log::Level::Debug => "\x1B[34m",
                    log::Level::Trace => "\x1B[90m",
                },
                level = record.level(),
                reset = "\x1B[0m",
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("actix_server", log::LevelFilter::Warn)
        .level_for("actix_web", log::LevelFilter::Warn)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}