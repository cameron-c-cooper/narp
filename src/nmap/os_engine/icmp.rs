use std::{io::Write, net::{IpAddr, Ipv4Addr, SocketAddr}};
use std::time::{Duration, SystemTime};
use etherparse::Ipv4Header;
use pnet::packet::icmp::IcmpPacket;
use thiserror::Error;
use socket2::{Type, Protocol, Domain, Socket};

pub const HEADER_SIZE: usize = 8;

pub struct IcmpV4;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid size")]
    InvalidSize,
    #[error("invalid packet")]
    InvalidPacket,
}

pub trait Proto {
    const ECHO_REQUEST_TYPE: u8;
    const ECHO_REQUEST_CODE: u8;
    const ECHO_REPLY_TYPE: u8;
    const ECHO_REPLY_CODE: u8;
}

impl Proto for IcmpV4 {
    const ECHO_REQUEST_TYPE: u8 = 8;
    const ECHO_REQUEST_CODE: u8 = 0;
    const ECHO_REPLY_TYPE: u8 = 0;
    const ECHO_REPLY_CODE: u8 = 0;
}

pub struct EchoRequest<'a> {
    pub id: u16,
    pub seq: u16,
    pub payload: &'a [u8]
}

impl <'a> EchoRequest<'a> {
    pub fn encode<P:Proto>(&self, buffer: &mut [u8]) -> Result<(), Error> {
        buffer[0] = P::ECHO_REQUEST_TYPE;
        buffer[1] = P::ECHO_REQUEST_CODE;


        buffer[4] = (self.id >> 8) as u8;
        buffer[5] = self.id as u8;
        buffer[6] = (self.seq >> 8) as u8;
        buffer[7] = self.seq as u8;

        // First 8 bytes are id and seq, as well as echo req type and code
        if let Err(_) = (&mut buffer[8..]).write_all(self.payload) {
            return Err(Error::InvalidSize);
        } 
        write_checksum(buffer);
        Ok(())
    }
}

pub struct EchoReply<'a> {
    pub id: u16,
    pub seq: u16,
    pub payload: &'a [u8]
}

impl <'a> EchoReply<'a> {
    pub fn decode<P: Proto>(buffer: &'a [u8]) -> Result<Self, Error> {
        if buffer.as_ref().len() < HEADER_SIZE {
            return Err(Error::InvalidSize);
        }

        let type_ = buffer[0];
        let code = buffer[1];

        if type_ != P::ECHO_REPLY_TYPE || code != P::ECHO_REPLY_CODE {
            return Err(Error::InvalidPacket);
        }

        let id = (u16::from(buffer[4]) << 8) + u16::from(buffer[5]);
        let seq = (u16::from(buffer[6]) << 8) + u16::from(buffer[7]);
        let payload = &buffer[HEADER_SIZE..];

        Ok(EchoReply {
            id,
            seq,
            payload
        })
    }
}

pub fn write_checksum(buffer: &mut [u8]) {
    let mut sum = 0u32;
    for word in buffer.chunks(2) {
        let mut part = u16::from(word[0]) << 8;
        if word.len() > 1 {
            part += u16::from(word[1]);
        }
        sum = sum.wrapping_add(u32::from(part));
    }

    while (sum >> 16) > 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }

    let sum = !sum as u16;

    buffer[2] = (sum >> 8) as u8;
    buffer[3] = (sum & 0xff) as u8;
}

const TOKEN_SIZE: usize = 24;
const ECHO_REQUEST_BUFFER_SIZE: usize = HEADER_SIZE + TOKEN_SIZE;
type Token = [u8; TOKEN_SIZE];

pub fn ping_with_sock_type(
    sock_t: Type,
    addr: IpAddr,
    timeout: Option<Duration>,
    ttl: Option<u32>,
    id: Option<u16>,
    seq: Option<u16>,
    payload: Option<&Token>
) -> Result<(), Error> {
    // let time_start = SystemTime::now();
    let timeout = match timeout {
        Some(timeout) => timeout,
        None => Duration::from_secs(4),
    };

}
