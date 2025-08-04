use std::{collections::HashSet, sync::{Arc, Mutex}, thread};
use log::info;
use pnet::{datalink::{self, Channel, Config, NetworkInterface}, packet::ethernet::EthernetPacket};

pub fn device_listener_init() {
    let interfaces: Vec<NetworkInterface> = datalink::interfaces()
        .into_iter()
        .filter(|i| i.is_up() && !i.is_loopback() && !i.ips.is_empty())
        .collect();
    interfaces.iter().for_each(|i| info!("Interface Discovered: \n      {i}"));
    let seen = Arc::new(Mutex::new(HashSet::new()));
    for iface in interfaces {
        let iface_name = iface.name.clone();
        let seen = seen.clone();
        let mut config = Config::default();
        config.promiscuous = true;

        match datalink::channel(&iface, config) {
            Ok(Channel::Ethernet(mut _tx, mut rx)) => {
                thread::spawn(move || {
                    info!("[{}] listening...", iface_name);
                    while let Ok(packet) = rx.next() {
                        if let Some(eth) = EthernetPacket::new(packet) {
                            let src_mac = eth.get_source();
                            let mut seen = seen.lock().unwrap();
                            if seen.insert(src_mac) {
                                info!(
                                    "[{}] New MAC detected: {} (EtherType: 0x{:04x})",
                                    iface_name,
                                    src_mac,
                                    eth.get_ethertype().0
                                );
                            }
                        }
                    };
                })
            },
            Ok(_) => panic!("[{}] Unhandled Channel Type", iface_name),
            Err(e) => panic!("[{}] Failed to open channel: {}", iface_name, e)
        };
    }
    loop {
        thread::park();
    }
}
