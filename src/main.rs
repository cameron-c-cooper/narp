extern crate simplelog;

use log::{self, debug, error, info, trace, warn};
use std::net::Ipv4Addr;
use narp::{self, init_term_logging, os_detection::get_open_ports};

fn main() {
    init();
}

fn init() {
    init_term_logging();
}
