use pnet::{
    packet::{
        tcp::Tcp,
    },
};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};

struct TcpProbes {
    probes: [Probe; 6],
    dest: Ipv4Addr
}

struct Probe {
    seq: u32,
    ack: u32,
    port: u16
}

impl Probe {
    fn send(&self, port: u16) {
        
    }
}

