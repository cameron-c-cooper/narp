use std::{net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream}, sync::mpsc::channel, time::Duration};

use threadpool::ThreadPool;



mod tcp;
mod icmp;
mod udp;

pub fn get_os(addr: Ipv4Addr) -> String {
    let open_tcp_ports = get_open_ports(addr);
    unimplemented!();
}

pub fn get_open_ports(addr: Ipv4Addr) -> Vec<u16> {
    let ports_per_thread = 200;
    let num_threads = tcp::TCP_COMMON_PORTS.len() as u16/ports_per_thread;

    let tp = ThreadPool::new(num_threads as usize);

    let (tx, rx) = channel();

    for i in 0..num_threads {
        let ports = &tcp::TCP_COMMON_PORTS[(i * ports_per_thread) as usize .. ((i + 1) * ports_per_thread) as usize];
        let tx = tx.clone();

        tp.execute(move || {
            let open_ports = scan_ports(addr, ports);
            let _ = tx.send(open_ports);
        });
    }
    drop(tx);
    let mut open_ports = Vec::new();
    for ports in rx {
        for port in ports {
            open_ports.push(port);
        }
    }
    open_ports
}

fn scan_ports(addr: Ipv4Addr, ports: &[u16]) -> Vec<u16> {
    let mut open_ports: Vec<u16> = Vec::new();

    for port in ports {
        if TcpStream::connect_timeout(
            &SocketAddr::new(IpAddr::V4(addr), port.to_owned()),
            Duration::from_millis(500),
        ).is_ok() {
            open_ports.push(port.to_owned());
        }
    };

    open_ports
}

#[test]
fn test_host_ports() {
    let ports = get_open_ports(Ipv4Addr::new(127, 0, 0, 1));
    assert_eq!(ports.len(), 0);
}



