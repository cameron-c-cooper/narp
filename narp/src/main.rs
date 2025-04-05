extern crate simplelog;

use core::panic;
use std::process::exit;

use log::{self, debug, error, info, trace, warn};
use narp;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // TODO: Add setup guide and maybe a script to walk through the process.
    let narp_url = std::env::var("NARP_URL").expect("NARP_URL Environment variable not set. Check the Git Repo for guide to set up narp.");

    let pool = match PgPoolOptions::new()
        .max_connections(15)
        .connect(&narp_url)
        .await {
            Ok(p) => p,
            Err(_) => {
                error!("Failed to connect to database at {:?}", narp_url);
                panic!();
            }
        };
    debug!("Connected to database at {:?}", narp_url);
    info!("Narp network monitor started.");


    Ok(())
}

fn postgre_exists() -> bool {

}
