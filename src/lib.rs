use log::info;
use simplelog::{Config, SimpleLogger, TermLogger};
use socket2::Protocol;
use std::{
    mem,
    net::{IpAddr, Ipv4Addr},
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    net::{TcpStream, UdpSocket},
    spawn,
    task::JoinHandle,
    time::timeout,
};

pub mod database;
pub mod os_detection;
pub mod utils;

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

#[derive(Debug, PartialEq, Eq)]
pub struct Target {
    pub addr: IpAddr,
    pub addr_type: (bool, bool),
    pub os: Option<&'static str>,
    pub tpts: Arc<[Port]>,
    pub upts: Option<Arc<[Port]>>,
    pub tpo: Option<Arc<[Port]>>,
    pub upo: Option<Arc<[Port]>>,
}

type Port = u16;

impl Target {
    pub fn new_target_self(tpts: Arc<[Port]>, upts: Option<Arc<[Port]>>) -> Self {
        Target {
            addr: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            addr_type: (true, false),
            os: None,
            tpts,
            upts,
            tpo: None,
            upo: None,
        }
    }

    pub async fn scan_target(&mut self) -> std::io::Result<()> {
        info!("Scanning target {}", self.addr);
        let tcp_vec: Mutex<Vec<Port>> = Mutex::new(Vec::new());
        let udp_vec: Mutex<Vec<Port>> = Mutex::new(Vec::new());

        let mut handles: Vec<JoinHandle<Option<Port>>> = Vec::new();

        for port in self.tpts.iter() {
            let handle = spawn(scan_port(self.addr, *port, Protocol::TCP));
            handles.push(handle);
        }

        for handle in handles {
            if let Some(p) = handle.await.unwrap() {
                tcp_vec.lock().unwrap().push(p);
            }
        }

        let tcp_arc: Arc<[u16]> = {
            let mut lock = tcp_vec.lock().unwrap();
            let vec = mem::take(&mut *lock);
            Arc::from(vec)
        };

        self.tpo = Some(tcp_arc);

        if let Some(upts_arc) = self.upts.clone() {
            let mut handles: Vec<JoinHandle<Option<Port>>> = Vec::new();
            for port in upts_arc.iter() {
                let handle = spawn(scan_port(self.addr, *port, Protocol::UDP));
                handles.push(handle);
            }

            for handle in handles {
                if let Some(p) = handle.await.unwrap() {
                    udp_vec.lock().unwrap().push(p);
                }
            }

            let udp_arc: Arc<[u16]> = {
                let mut lock = udp_vec.lock().unwrap();
                let vec = mem::take(&mut *lock);
                Arc::from(vec)
            };

            self.upo = Some(udp_arc);
        }

        Ok(())
    }
}

async fn scan_port(addr: IpAddr, port: Port, proto: Protocol) -> Option<Port> {
    let target = format!("{}:{}", addr.to_string(), port);
    if proto == Protocol::TCP {
        match TcpStream::connect(target).await {
            Ok(_) => return Some(port),
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
