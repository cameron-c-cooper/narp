use std::{net::Ipv4Addr, time::Duration};
use pistol::{self, os_detect, Host, Target};
use subnetwork::CrossIpv4Pool;
pub fn get_os(addr: Ipv4Addr) -> String {
    unimplemented!();
}

pub struct Device {
    addr: Ipv4Addr,
    port_list: Vec<u16>,
    os_name: String
}

