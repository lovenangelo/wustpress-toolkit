use diesel::{Connection, MysqlConnection};
use dotenvy::dotenv;
use std::env;

fn load_config() -> Result<(), Box<dyn std::error::Error>> {
    // Try to load .env file
    match dotenv() {
        Ok(path) => println!("Loaded .env from: {:?}", path),
        Err(_) => println!("No .env file found, using system environment"),
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    println!("All required environment variables are set");
    Ok(())
}

pub fn establish() {
    let _ = load_config();
}
