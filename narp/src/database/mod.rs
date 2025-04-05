use std::{str::FromStr, sync::Arc};

use log::{error, info, warn};
use pnet::util::MacAddr;
use queues::Queue;
use sqlx::{self, Connection, PgConnection};
use crate::{Port, SqlDevice, Target};

pub const DATABASE_URL: &str = "postgres://narp_usr:narp_pswd@localhost/narp_db";

pub fn db_init() -> std::io::Result<()> {


    Ok(())
}

// TODO: Implement Target::to_sql_device()
pub async fn insert_sql_device(pool: sqlx::PgPool, device: &SqlDevice) -> Result<(), sqlx::Error> {
    let device = device.clone();
    if let Some(os) = device.os {
        sqlx::query("INSERT INTO devices (ip_addr, mac_addr, os, iface, ts) VALUES ($1, $2, $3, $4, $5)")
            .bind(device.ip_addr)
            .bind(device.mac_addr)
            .bind(os)
            .bind(device.iface)
            .bind(device.ts)
            .execute(&pool)
            .await?;
        info!("Inserted device of ")
        Ok(())
    } else {
        sqlx::query("INSERT INTO devices (ip_addr, mac_addr, iface, ts) VALUES ($1, $2, $3, $4)")
            .bind(device.ip_addr)
            .bind(device.mac_addr)
            .bind(device.iface)
            .bind(device.ts)
            .execute(&pool)
            .await?;
        Ok(())
    }
}

pub async fn get_device_by_id(pool: &sqlx::PgPool, device_id: i64) -> Result<SqlDevice, sqlx::Error> {
    let device = sqlx::query_as::<_, SqlDevice>("SELECT * FROM devices where id = $1")
        .bind(device_id)
        .fetch_one(pool)
        .await?;
    Ok(device)
}

pub async fn update_device(pool: &sqlx::PgPool, device: &SqlDevice, device_id: i64) -> Result<(), sqlx::Error> {
    unimplemented!();
}
