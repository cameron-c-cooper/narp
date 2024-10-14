use std::net::{Ipv4Addr};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use pnet_datalink::{interfaces, NetworkInterface, MacAddr};



pub struct arp {
    interfaces: Vec<NetworkInterface>,
    ip_addrs: Vec<Ipv4Addr>,
    mac_addrs: Vec<MacAddr>,
}


