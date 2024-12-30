use simplelog::{Config, SimpleLogger, TermLogger};

pub mod os_detection;
pub mod database;

pub fn init_term_logging() {
    let _ = TermLogger::init(
        log::LevelFilter::Trace,
        Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto
    );
}

pub fn init_logging() {
    let _ = SimpleLogger::init(
        log::LevelFilter::Trace,
        Config::default()
    );
}
