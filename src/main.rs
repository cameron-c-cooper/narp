extern crate simplelog;

use log::{self, debug, error, info, trace, warn};
use narp::{self, database, init_term_logging};
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    narp::init();

    Ok(())
}
