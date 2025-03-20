use std::{net::{IpAddr, Ipv4Addr, TcpListener}, sync::Arc, thread::{self, sleep}, time::Duration};

use narp::{database::{self, DB_LOC, insert_targets}, utils::TOP_TCP, Target};
use pnet::{datalink::interfaces, util::MacAddr};
use rusqlite::Connection;


#[test]
fn test_db_creation() {
    database::create_db().unwrap();
}

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
async fn test_target_insertion() {
    thread::spawn(|| {start_test_server(4000).unwrap();});
    thread::sleep(Duration::from_millis(500));
    let ifaces = interfaces();
    let iface = ifaces.into_iter().find(|iface| iface.is_up()).unwrap();
    let iface = Arc::new(iface);
    let mut t1 = Target::new(
        IpAddr::V4(Ipv4Addr::new(127,0,0,1)),
        MacAddr::new(0xff, 0xff, 0xff, 0xff, 0xff, 0xff),
        Arc::new(TOP_TCP),
        None,
        iface.clone()
    );
    let mut t2 = Target::new(
        IpAddr::V4(Ipv4Addr::new(127,0,0,1)),
        MacAddr::new(0xff, 0xff, 0xff, 0xff, 0xff, 0xff),
        Arc::new(TOP_TCP),
        None,
        iface.clone()
    );

    insert_targets(vec![t1, t2]).await.unwrap(); }
