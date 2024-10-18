use std::{collections::VecDeque, net::{
    IpAddr, Ipv4Addr, SocketAddr, TcpStream
}, sync::{mpsc::{Receiver, Sender}, Arc, Barrier}, usize};
use std::time::Duration;
use std::process;
use std::sync::mpsc::channel;
use std::sync::atomic::{AtomicUsize, Ordering};
use threadpool::ThreadPool;
use bit_set::BitSet;
use syslog::{Facility, Formatter3164};

pub mod os_engine;
mod utils;
use utils::PORTS_ARR;

pub struct NmapController {
    scan_queue: VecDeque<Ipv4Addr>,
    max_queue_size: u32,
}

pub enum NmapErrors {
    QueueMaxExceeded,
}

#[allow(unused)]
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
fn scan_ip(addr: IpAddr) -> BitSet {
    let port_range: Vec<u16> = (1..65535).collect();
    let formatter = Formatter3164 {
        facility: Facility::LOG_DAEMON,
        hostname: None,
        process: "narp".into(),
        pid: process::id(),
    };
    // PORTS_ARR is the array containing the 1000 most frequent TCP ports
    let num_jobs = PORTS_ARR.len();
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
                    0..=0xffff => bitset.insert(index),

                    _ => false,
                };
            },
            Err(e) => {
                let formatter = formatter.clone();
                match syslog::unix(formatter) {
                    Err(e) => println!("impossible to connect to syslog: {:?}", e),
                    Ok(mut writer) => {
                        writer.info("no more ports").expect("could not write error message");
                    }
                }
            }
        }
    }

    return bitset;
}

fn scan_for_os(addr: IpAddr, ports: Vec<u16>) -> String {
    let num_jobs = ports.len();
    let tp = ThreadPool::new(num_jobs);
    // +1 represents the final thread which will call wait
    let barrier = Arc::new(Barrier::new(num_jobs + 1));
    for _ in 0..num_jobs {
    }
    String::new()
}

