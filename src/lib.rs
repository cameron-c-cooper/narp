use log::info;
use pnet::{datalink::{interfaces, NetworkInterface}, util::MacAddr};
use simplelog::{Config, SimpleLogger, TermLogger};
use socket2::Protocol;
use std::{
    mem,
    net::{IpAddr, Ipv4Addr},
    sync::{Arc, Mutex},
    time::Duration, u16,
};
use tokio::{
    net::{TcpStream, UdpSocket},
    spawn,
    task::JoinHandle,
    time::timeout,
};

extern crate queues;

pub mod database;
pub mod os_detection;
pub mod utils;
pub mod target_detection;

pub fn init() {
    info!("Narp daemon started");
    init_term_logging();
    let _ = database::create_db();
}

pub fn init_term_logging() {
    let _ = TermLogger::init(
        log::LevelFilter::Trace,
        Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    );

    info!("TermLogging initialized")
}

pub fn init_logging() {
    let _ = SimpleLogger::init(log::LevelFilter::Trace, Config::default());

    info!("Simple logging initialized");
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Target {
    pub ip_addr: IpAddr,
    pub mac_addr: MacAddr,
    pub os: Option<&'static str>,
    pub tpts: Arc<[u16]>,
    pub upts: Option<Arc<[u16]>>,
    pub tpo: Option<Arc<[Port]>>,
    pub upo: Option<Arc<[Port]>>,
    pub iface: Arc<NetworkInterface>, // TODO: Check if this should be Arc or rc, unsure if multiple
                                     // threads will be accessing this
    pub inserted: bool
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Port {
    pub idx: u16,
    pub service: Option<&'static str>,
}

impl Port {
    pub fn new(idx: u16, service: Option<&'static str>) -> Self {
        Port {
            idx,
            service
        }
    }

    pub fn set_service(&mut self, service: &'static str) {
        self.service = Some(service);
    }
}

impl Target {
    pub fn new(addr: IpAddr, mac_addr: MacAddr, tpts: Arc<[u16]>, upts: Option<Arc<[u16]>>, iface: Arc<NetworkInterface>) -> Self {
        Target {
            ip_addr: addr,
            mac_addr,
            os: None,
            tpts,
            upts,
            tpo: None,
            upo: None,
            iface,
            inserted: false
        }
    }

    pub fn new_target_self(tpts: Arc<[u16]>, upts: Option<Arc<[u16]>>) -> Self {
        let iface = Arc::new(interfaces().iter().find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty()).unwrap().clone());
        
        Target {
            ip_addr: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            mac_addr: MacAddr::new(0xff, 0xff, 0xff, 0xff, 0xff, 0xff),
            os: None,
            tpts,
            upts,
            tpo: None,
            upo: None,
            iface,
            inserted: false
        }
    }

    /**
     * scans all ports on a target and mutates the tpo/upo ports.
     * tcp ports are ALWAYS checked, udp ports sometimes depending
     * on the configuration of the target when it is created. Thought
     * about immediately launching the os engine from this fn, but
     * figured it would be better to just do it manually, having the
     * os engine explicitly run on another thread where there is a
     * shared queue.
     */
    pub async fn scan(&mut self) -> std::io::Result<()> {
        info!("Scanning target {}", self.ip_addr);
        let tcp_vec: Mutex<Vec<Port>> = Mutex::new(Vec::new());
        let udp_vec: Mutex<Vec<Port>> = Mutex::new(Vec::new());

        let mut handles: Vec<JoinHandle<Option<Port>>> = Vec::new();

        for port in self.tpts.iter() {
            let handle = spawn(scan_port(self.ip_addr, *port, Protocol::TCP));
            handles.push(handle);
        }

        for handle in handles {
            if let Some(p) = handle.await.unwrap() {
                tcp_vec.lock().unwrap().push(p);
            }
        }

        let tcp_arc: Arc<[Port]> = {
            let mut lock = tcp_vec.lock().unwrap();
            let vec = mem::take(&mut *lock);
            Arc::from(vec)
        };

        self.tpo = Some(tcp_arc);

        if let Some(upts_arc) = self.upts.clone() {
            let mut handles: Vec<JoinHandle<Option<Port>>> = Vec::new();
            for port in upts_arc.iter() {
                let handle = spawn(scan_port(self.ip_addr, *port, Protocol::UDP));
                handles.push(handle);
            }

            for handle in handles {
                if let Some(p) = handle.await.unwrap() {
                    udp_vec.lock().unwrap().push(p);
                }
            }

            let udp_arc: Arc<[Port]> = {
                let mut lock = udp_vec.lock().unwrap();
                let vec = mem::take(&mut *lock);
                Arc::from(vec)
            };

            self.upo = Some(udp_arc);
        }

        Ok(())
    }

    /**
     * Returns values section of SQL DML statement
     */
    pub fn format_to_sql(&self) -> String {
        let mut target_vals = format!("('{}', '{}'", self.ip_addr, self.mac_addr);

        if let Some(os) = self.os {
            target_vals.push_str(&format!(", '{os}'"));
        }
        
        target_vals.push(')');
        target_vals
    }

    pub fn set_inserted(&mut self, b: bool) {
        self.inserted = b;
    }
}

async fn scan_port(addr: IpAddr, port: u16, proto: Protocol) -> Option<Port> {
    let target = format!("{}:{}", addr.to_string(), port);
    if proto == Protocol::TCP {
        match TcpStream::connect(target).await {
            Ok(_) => return Some(Port::new(port, None)),
            Err(_) => return None,
        }
    } else if proto == Protocol::UDP {
        let sock = match UdpSocket::bind("0.0.0.0:80").await {
            Ok(s) => s,
            Err(_) => return None,
        };

        let payload = b"        ";

        if sock.send_to(payload, target).await.is_err() {
            return None;
        }

        let mut buf = [0; 1024];

        match timeout(Duration::from_secs(1), sock.recv_from(&mut buf)).await {
            Ok(Ok(_)) => return None,
            _ => return None,
        }
    }
    None
}

#[test]
fn test_socket_fmt() {
    let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port: u16 = 80;
    assert_eq!(format!("{}:{}", addr, port), "127.0.0.1:80");
}
