use log::error;
use rusqlite::{self, Connection, Result};

pub fn create_db() -> Result<()> {
    #[cfg(target_family = "unix")]
    let db_loc = "/var/lib/narp/narp.db";

    #[cfg(target_os = "macos")]
    let db_loc = "/Library/Application Support/narp/narp.db";

    #[cfg(target_os = "windows")]
    let db_loc = "%AppData%\\Local\\Low\\narp.db";

    #[cfg(target_os = "linux")]
    let db_loc = "/var/lib/narp/narp.db";

    let conn = match Connection::open(db_loc) {
        Ok(c) => c,
        Err(_) => {
            error!("Failed to create database at {db_loc}");
            panic!();
        }
    };

    let device_table_stmt = r#"
        create table if not exists devices (
            id integer primary key autoincrement,
            hostname text unique not null,
            operating_system text,
            ip_addr text,
            mac_addr text,
            ts datetime default current_timestamp
        )
    "#;

    let port_table_stmt = r#"
        create table if not exists ports (
            id integer primary key autoincrement,
            device_id integer not null,
            port_number integer not null,
            protocol text check(protocol in ('TCP', 'UDP')) not null,
            status text check(status in ('open', 'closed', 'filtered')) not null,
            last_checked datetime default current_timestamp,
            foreign key (device_id) references devices(id) on delete cascade
        )
    "#;
    let service_table_stmt = r#"
        create table if not exists services (
            id integer primary key autoincrement,
            port_id integer unique not null,
            service_name text not null,
            version text,
            extra_info text,
            foreign key (port_id) references ports(id) on delete cascade,
        )
    "#;
    let _ = conn.execute(device_table_stmt, []);
    let _ = conn.execute(port_table_stmt, []);
    let _ = conn.execute(service_table_stmt, []);

    Ok(())
}
