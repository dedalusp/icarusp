//! Icarusp Backend Library
//!
//! This library provides the core functionality for the Icarusp backend,
//! including database operations, embedding generation, and data models.

pub mod database;
pub mod embedding;
pub mod models;
pub mod schema;

use anyhow::Result;
use database::initialization::establish_connection;
use diesel::PgConnection;

/// Initialize the application and test database connection
pub fn initialize() -> Result<()> {
    dotenvy::dotenv().ok();
    // Test that we can establish a connection
    let _conn = establish_connection()?; // This is now correct as establish_connection returns Result
    Ok(())
}

/// Get a database connection
pub fn get_connection() -> PgConnection {
    // You need to decide how to handle the error here.
    // For now, let's unwrap it, but in a real application, you'd want more robust error handling.
    establish_connection().expect("Failed to establish database connection")
}

/// Application configuration
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/diesel_demo".to_string());

        let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let server_port = std::env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .map_err(|e| anyhow::anyhow!("Invalid SERVER_PORT: {}", e))?;

        Ok(Config {
            database_url,
            server_host,
            server_port,
        })
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env() {
        // This test will use default values if env vars are not set
        let config = Config::from_env();
        assert!(config.is_ok());

        let config = config.unwrap();
        assert!(!config.database_url.is_empty());
        assert!(!config.server_host.is_empty());
        assert!(config.server_port > 0);
    }

    #[test]
    fn test_bind_address() {
        let config = Config {
            database_url: "test".to_string(),
            server_host: "localhost".to_string(),
            server_port: 3000,
        };
        assert_eq!(config.bind_address(), "localhost:3000");
    }
}
