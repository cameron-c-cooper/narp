use std::net::Ipv4Addr;

use narp::{self, os_detection::get_open_ports};
fn main() {
    println!("Establishing Connection...");

    get_open_ports(Ipv4Addr::new(127, 0, 0, 1));
}
