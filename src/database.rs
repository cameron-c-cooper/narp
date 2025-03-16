use std::{str::FromStr, sync::Arc};

use log::{error, info};
use rusqlite::{self, Connection, Result};

use crate::{Port, Target}; 

// #[cfg(target_os = "macos")]
// let db_loc = "/Library/Application Support/narp/narp.db";

#[cfg(target_os = "windows")]
pub const DB_LOC: &str = "%AppData%\\Local\\Low\\narp.db";

// #[cfg(all(target_os = "linux", not(test)))]
// pub const DB_LOC: &str = "/var/lib/narp/narp.db";

#[cfg(all(target_os = "macos"))]
pub const DB_LOC: &str = "/Users/558632/dev/narp/db_testing/narp.db";

#[cfg(target_os = "linux")]
pub const DB_LOC: &str = "/home/kinveth/dev/narp/db_testing/narp.db";

pub fn create_db() -> Result<()> {
    let conn = match Connection::open(DB_LOC) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to create database at {DB_LOC}");
            return Err(e);
        }
    };

    let device_table_stmt = r#"
        create table if not exists devices (
            id integer primary key autoincrement,
            host_name text,
            ip_addr text,
            mac_addr text,
            operating_system text,
            ts datetime default current_timestamp
        )
    "#;

    let port_table_stmt = r#"
        create table if not exists ports (
            id integer primary key autoincrement,
            device_id integer not null,
            port_number integer not null,
            service_running text,
            protocol text check(protocol in ('TCP', 'UDP')) not null,
            status text check(status in ('open', 'closed', 'filtered')) not null,
            last_checked datetime default current_timestamp,
            foreign key (device_id) references devices(id) on delete cascade
        )
    "#;

    let _ = conn.execute(device_table_stmt, []);
    let _ = conn.execute(port_table_stmt, []);

    Ok(())
}

/**
 * Assumes all targets have os or no os, no mixing
 */
pub fn insert_targets(targets: Vec<Target>) -> Result<()> {
    let mut base_stmt = format!("insert into devices (ip_addr, mac_addr");
    let mut vals = String::from_str(" values ").unwrap(); // unwrap bc if this panics thats jsut
                                                          // fucking crazy
    let conn = match Connection::open(DB_LOC) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to open database at {DB_LOC}");
            return Err(e);
        }
    };
    if targets[0].os.is_some() {
        base_stmt.push_str(", operating_system)");
        for t in targets {
            vals.push_str(&format!("{},", t.format_to_sql()));
        }
        vals.pop();
    } else {
        base_stmt.push(')');
        for t in targets {
            vals.push_str(&format!("{},", t.format_to_sql()));
        }
    }
    base_stmt.push_str(&vals);
    base_stmt.pop();
    println!("{base_stmt}");
    match conn.execute(&base_stmt, []) {
        Ok(_) => {
            info!("Inserted new target to database");
            return Ok(())
        }
        Err(e) => {
            error!("Inserting target failed: {e}");
            return Err(e);
        }
    };
}

pub fn insert_ports() {

}
