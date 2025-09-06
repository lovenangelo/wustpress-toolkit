use dotenvy::dotenv;
use std::env;

fn load_config() -> Result<(), Box<dyn std::error::Error>> {
    // Try to load .env file
    match dotenv() {
        Ok(path) => println!("Loaded .env from: {:?}", path),
        Err(_) => println!("No .env file found, using system environment"),
    }

    // Validate required environment variables
    let required_vars = [
        "DATABASE_HOST",
        "DATABASE_SOURCE_NAME",
        "DATABASE_PORT",
        "DATABASE_USER",
        "DATABASE_PASSWORD",
    ];

    for var in &required_vars {
        env::var(var).map_err(|_| panic!("Required environment variable {} is not set", var))?;
    }

    println!("All required environment variables are set");
    Ok(())
}

pub fn initialize() {
    let _ = load_config();
}
