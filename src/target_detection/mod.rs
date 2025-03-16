use std::sync::{Arc, Mutex};

use log::{debug, info, log, warn};
use pnet::{self, datalink::{self, interfaces, Channel, NetworkInterface}, packet::{self, arp::{self, ArpOperation, ArpOperations, ArpPacket}, ethernet::{self, EtherTypes, EthernetPacket}, Packet}};
use queues::Queue;
use tokio::spawn;

use crate::Target;

/*
 * TODO: Think about having a local hashmap to allow O(1) checking if
 * target already exists. This should be faster than accessing the SQL
 * database, but if space becomes a concern then just stick with the
 * sql database
 * */
pub fn arp_req_listener(target_queue: Arc<Mutex<Queue<Target>>>) {
    let ifaces = interfaces();
    let mut valid_ifaces: Vec<NetworkInterface> = Vec::new();
    for i in 0..ifaces.len() {
        if !ifaces[i].ips.is_empty() && !ifaces[i].is_loopback() && ifaces[i].is_up() {
            valid_ifaces.push(ifaces[i].clone());
        }
    }

    for iface in valid_ifaces {
        let target_queue = target_queue.clone();
        spawn(async move {

            let (_, mut rx) = match datalink::channel(&iface, Default::default()) {
                Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
                Ok(_) => panic!("Unsupported channel type on {:?}", iface),
                Err(e) => panic!("Failed to create datalink channel on {:?}", e)
            };

            info!("Listening for ARP packets on {:?}", iface.name);

            match rx.next() {
                Ok(packet) => {
                    let _ = arp_packet_handler(packet, target_queue.clone());
                },
                Err(e) => {
                    warn!("Failed to read packet on {:?}, breaking out of loop. \n Error: {}", iface, e);
                }
            }
        });
    }
}

pub fn arp_packet_handler(data: &[u8], target_queue: Arc<Mutex<Queue<Target>>>) -> std::io::Result<()> {
    if let Some(eth_packet) = EthernetPacket::new(data) {
        if eth_packet.get_ethertype() == EtherTypes::Arp {
            if let Some(arp_packet) = ArpPacket::new(eth_packet.payload()) {
                let op = arp_packet.get_operation();
                match op {
                    ArpOperations::Request => {
                        let target = arp_packet.get_target_proto_addr();
                        let sender = arp_packet.get_sender_proto_addr();
                        info!("ARP Request detected: \n\tTarget: {}\n\tSender: {}", target, sender);
                    },
                    ArpOperations::Reply => {
                        let target = arp_packet.get_target_proto_addr();
                        let sender = arp_packet.get_sender_proto_addr();
                        info!("ARP Reply detected: \n\tTarget: {}\n\tSender: {}", target, sender);

                        if target_exists()? {
                            todo!();
                        }
                    },
                    _ => debug!("Unsupported ARP operation: {:?}", op)
                }
            }
        }
    }

    Ok(())
}

fn target_exists() -> std::io::Result<bool> {
    Ok(true)
}
