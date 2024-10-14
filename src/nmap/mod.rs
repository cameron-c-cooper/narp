use std::{collections::VecDeque, net::{
    IpAddr, Ipv4Addr, SocketAddr, TcpStream
}, sync::mpsc::{Receiver, Sender}, thread::Thread, usize};
use std::time::Duration;
use std::process;
use std::sync::mpsc::channel;
use std::sync::atomic::{AtomicUsize, Ordering};
use threadpool::ThreadPool;
use bit_set::BitSet;
use syslog::{Facility, Formatter3164};

mod utils;
use utils::PORTS_ARR;

pub struct NmapController {
    scan_queue: VecDeque<Ipv4Addr>,
    max_queue_size: u32,
}

pub enum NmapErrors {
    QueueMaxExceeded,
}

impl NmapController {
    pub fn new(max_len_queue: Option<u32>) -> NmapController {
        let default_max_queue_size: u32 = 500; 
        let max_queue_size: u32 = match max_len_queue {
            Some(v) => v,
            _ => default_max_queue_size,
        };
        let scan_queue: VecDeque<Ipv4Addr> = VecDeque::with_capacity(100);
        NmapController {
            scan_queue,
            max_queue_size,
        }
    }

    pub fn recv_new_items(&mut self, mut queue: VecDeque<Ipv4Addr>) -> Result<u32, NmapErrors> {
        if queue.len() + self.scan_queue.len() > self.max_queue_size as usize {
            return Err(NmapErrors::QueueMaxExceeded)
        } else {
            self.scan_queue.append(&mut queue);
            Ok(self.scan_queue.len() as u32)
        }
    }
}


/** Yeah so this is memory heavy. like HEAVY. Each port is represented as 1 or 0 on a bit array.
 * This is likely highly inefficient and I would like to change this in the future, but for now
 * that is what I will do.
 */
#[allow(unused)]
pub fn scan_ip(addr: IpAddr) -> BitSet {
    let port_range: Vec<u16> = (1..65535).collect();
    // There are 500 jobs per worker, excluding the last one.
    let num_jobs = PORTS_ARR.len();
    let mut open_ports: Vec<u16> = Vec::new();
    let total_ports = num_jobs;
    let tp = ThreadPool::new(100);
    let mut bitset: BitSet = BitSet::with_capacity(PORTS_ARR.len());
    let (tx, rx): (Sender<u32>, Receiver<u32>) = channel();
    for job in 0..num_jobs {
        let tx = tx.clone();
        tp.execute(move|| {
            let connection_with_timeout = TcpStream::connect_timeout(&SocketAddr::new(addr, PORTS_ARR[job]), Duration::from_secs(1));
            match connection_with_timeout {
                Ok(_) => tx.send(job as u32),
                Err(_) => tx.send(u32::MAX),
            };
        });
    };
    for index in 0..num_jobs {
        match rx.recv() {
            Ok(num) => {
                match num {
                    0..=u32::MAX => bitset.insert(index),
                    _ => false,
                };
            },
            Err(e) => {

            }
        }
    }

    return bitset;
}


