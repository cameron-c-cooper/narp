use chrono::SecondsFormat;
use fern::Dispatch;
use log::{debug, info, log};

use crate::device_listener::device_listener_init;

mod device_listener;

fn main() {
    init_logging().unwrap();
    device_listener_init();
}

fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                    "[{} {} {}] {}",
                    chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
                    record.level(),
                    record.target(),
                    message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    info!("Fern Logging Initialized");
    Ok(())
}
