extern crate simplelog;

use log::{self, debug, error, info, trace, warn};
use narp::{self, database, init_term_logging};
use std::net::Ipv4Addr;

fn main() {
    init();
}

fn init() {
    init_term_logging();
    database::create_db();
}
