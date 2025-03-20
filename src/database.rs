use std::{str::FromStr, sync::Arc};

use log::{error, info, warn};
use pnet::util::MacAddr;
use queues::Queue;
use sqlx::{self, Connection, PgConnection};
use crate::{Port, Target}; 

pub const DATABASE_URL: &str = "postgres://narp_usr:narp_pswd@localhost/narp_db";

pub async fn db_init() -> std::io::Result<()> {
    let init_stmt = r#"
        SELECT 'CREATE DATABASE narp_db'
        WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname='narp_db')\gexec
        "#;

    let conn = PgConnection::connect(DATABASE_URL).await.;


    Ok(())
}
