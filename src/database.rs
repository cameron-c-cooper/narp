use sqlite::{open, Connection};
use std::net::Ipv4Addr;
use std::path::Path;
use std::process;
use std::collections::VecDeque;

crate::nmap;
crate::arp;

struct DBManager {
    conn: Connection,
    path: String,
    wants_more: Vec<Ipv4Addr>,
}

impl DBManager {
    pub fn new(db_path: Option<String>) -> DBManager {
        let default_path = "/var/lib/narp/narp.db";
        let path = match db_path {
            Some(s) => s,
            _ => default_path.to_string(),
        };
        let conn: Connection = match sqlite::open(path.clone()) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error with opening sqlite database: {}", e);
                process::exit(1);
            }
        };
        let wants_more: Vec<Ipv4Addr> = Vec::new();

        DBManager {
            conn,
            path,
            wants_more,
        }
    }

    pub fn query_nmap(nmap: ) 
}
