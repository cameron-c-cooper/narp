use log::info;
use simplelog::{Config, SimpleLogger, TermLogger};

pub mod database;
pub mod os_detection;

pub fn init_term_logging() {
    let _ = TermLogger::init(
        log::LevelFilter::Trace,
        Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    );

    info!("TermLogging initialized")
}

pub fn init_logging() {
    let _ = SimpleLogger::init(log::LevelFilter::Trace, Config::default());

    info!("Simple logging initialized");
}
