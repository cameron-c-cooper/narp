use std::{
    net::{IpAddr, Ipv4Addr, TcpListener},
    sync::Arc,
    thread, time::Duration
};

use narp::{
    utils::{self, TOP_TCP},
    Target,
};
use pnet::{datalink::interfaces, util::MacAddr};

fn start_test_server(port: u16) -> std::io::Result<()> {
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address)?;
    println!("Listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                // Handle incoming connections if needed
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_main() {
    thread::spawn(|| {start_test_server(4000).unwrap();});
    thread::sleep(Duration::from_millis(500));
    let ifaces = interfaces();
    for i in ifaces.clone() {
        println!("{:?}", i);
    }
    let iface = ifaces.into_iter().find(|iface| iface.is_up()).unwrap();
    let iface = Arc::new(iface);
    let tpts = Arc::new(utils::TOP_TCP);
    let mut target = Target::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), MacAddr::new(0xff, 0xff, 0xff, 0xff, 0xff, 0xff), tpts, None, iface.clone());
    target.scan().await.unwrap();
    if let Some(ports) = target.tpo {
        assert!(ports.len() > 0);
    } else {
        panic!();
    }
}
