extern crate simplelog;

use log::{self, debug, error, info, trace, warn};
use narp;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    narp::init(); // also spawns os detection engine

    Ok(())
}
