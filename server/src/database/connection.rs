use dotenvy::dotenv;
use mysql::{Opts, OptsBuilder, Pool, PooledConn, Result};
use std::env;
use std::sync::OnceLock;
// Global static pool instance
static DB_POOL: OnceLock<Pool> = OnceLock::new();

// Initialize the database pool
pub fn init_db_pool() -> Result<()> {
    match dotenv() {
        Ok(path) => println!("Loaded .env from: {:?}", path),
        Err(_) => println!("No .env file found, using system environment"),
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let database_url = database_url.as_str();

    // Configure connection options
    let opts = Opts::from_url(database_url)?;

    // Create the pool
    let pool = Pool::new(opts)?;

    // Store it in the global static
    DB_POOL
        .set(pool)
        .map_err(|_| mysql::Error::DriverError(mysql::DriverError::UnexpectedPacket))?;

    Ok(())
}

pub fn get_db_connection() -> Result<PooledConn> {
    let pool = DB_POOL.get().expect("Database pool not initialized");
    println!("Database connection established");
    pool.get_conn()
}
