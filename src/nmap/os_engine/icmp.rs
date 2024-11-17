use std::{net::{IpAddr, Ipv4Addr}, str::FromStr, time::Duration, mem};
use nix::{libc::{self, socket, PF_INET, SOCK_RAW}, NixPath};
use pnet::{
    packet::{
        icmp::{
            echo_reply::{self, EchoReplyPacket}, echo_request::MutableEchoRequestPacket, IcmpCode, IcmpTypes::EchoRequest
        }, ip::IpNextHeaderProtocols::{self, Icmp}, ipv4::{
            Ipv4Packet,
            MutableIpv4Packet
        }, FromPacket, Packet
    }, 
    transport::{self, icmp_packet_iter, TransportChannelType},
    util::checksum,
};
use rand::{thread_rng, Rng};
use local_ip_address::{linux::local_ip};


const windowSizes: [u16; 13] = [ 1, 63, 4, 4, 16, 512, 3, 128, 256, 1024, 31337, 32768, 65536 ];
/*
 * WINDOW SIZES
 *  1   63   4     4
 *  16  512  3     128
 *  256 1024 31337 32760
 *  65535
 */

/*
 * ICMP PACKET STRUCTURE
 *
 * 4 Version | 4 header | 8 Type of Service | 16 Total Length
 * 16 Identifier | 4 flags | 12 fragment offset
 * 8 Time to Live | 8 Protocol | 16 header checksum
 * 32 source addr
 * 32 dest addr
 * 8 type | 8 code | 16 icmp checksum
 * 32 data
 *
 */
enum IcmpError {
    Generic
}


const DF: u8 = 0b010;
const MF: u8 = 0b001;

fn get_local_ip_addr() -> Ipv4Addr {
    let str = local_ip().unwrap().to_string();
    Ipv4Addr::from_str(&str).unwrap()
}

/** MAKE SURE THE OK() RESULT IS USED AND MOVED TO EchoReplyPacket */
fn send_packet1(ip_addr: Ipv4Addr) -> Result<Vec<u8>, String> {
    let protocol = TransportChannelType::Layer3(IpNextHeaderProtocols::Icmp);
    // TODO move this to a match statement and syslog it. this is a daemon, not a user process
    let (mut tx, mut rx) = transport::transport_channel(1024, protocol)
        .expect("Failed to create transport channel");

    let mut buf = vec![0u8; 64];
    
    let id = thread_rng().gen();
    // single packet for now
    let mut packet = MutableEchoRequestPacket::new(&mut buf)
        .expect("Failed to create ICMP packet");
    packet.set_icmp_type(EchoRequest);
    packet.set_sequence_number(295);
    packet.set_icmp_code(IcmpCode::new(9));
    packet.set_identifier(id);
    // payload of 120 bytes of 0x0
    packet.set_payload(&[0u8; 120]);

    let ip_header_len = 20;
    let icmp_len = packet.packet().len();
    let total_len = ip_header_len + icmp_len;

    let mut raw_packet = vec![0u8; total_len];
    let splices = &mut raw_packet.split_at_mut(ip_header_len);
    let mut ip_packet = MutableIpv4Packet::new(&mut splices.0)
        .expect("Failed to create IP packet");
    ip_packet.set_version(4);
    ip_packet.set_header_length(ip_header_len as u8/4); // ip header length measured in words. Says
                                                        // u8 but it really is u4
                                                    
    ip_packet.set_total_length(total_len as u16);
    ip_packet.set_identification(id);
    ip_packet.set_flags(DF);
    ip_packet.set_ttl(64);
    ip_packet.set_next_level_protocol(Icmp);
    ip_packet.set_source(get_local_ip_addr());
    ip_packet.set_destination(ip_addr);
    
    
    let ip_payload = &mut splices.1;
    ip_payload.copy_from_slice(packet.packet());

    let ip_checksum = checksum(&ip_packet.packet(), 1);
    ip_packet.set_checksum(ip_checksum);

    tx.send_to(ip_packet, IpAddr::V4(ip_addr)).expect("Failed to send packet");
    
    let mut iter = icmp_packet_iter(&mut rx);
    let (reply, addr) = iter.next_with_timeout(Duration::from_millis(500)).expect("Failed to get reply packet").expect("Failed to get reply packet");


    if addr == IpAddr::from(ip_addr) {
        let mut v: Vec<u8> = Vec::new();
        for i in reply.packet().iter() {
            v.push(*i);
        }
        return Ok(v)
    }
    return Err("Failed to get reply".to_string())
}


