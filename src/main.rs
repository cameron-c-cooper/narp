extern crate simplelog;

use log::{self, debug, error, info, trace, warn};
use narp;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(15) // roughly 150-200 MB worth of connections
        .connect(url)
    narp::init(); // also spawns os detection engine

    Ok(())
}
