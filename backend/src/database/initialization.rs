use anyhow::Result;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env; // Import Result from anyhow

/// Establishes a direct database connection using Diesel
pub fn establish_connection() -> Result<PgConnection> {
    // Change return type to Result
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/diesel_demo".to_string());

    PgConnection::establish(&database_url)
        .map_err(|e| anyhow::anyhow!("Error connecting to {}: {}", database_url, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_establish_connection() {
        // This test requires a valid database connection
        // Skip if DATABASE_URL is not set properly
        if env::var("DATABASE_URL").is_err() {
            return;
        }

        let connection = establish_connection(); // No longer needs _with_error_handling, as it returns Result
        assert!(connection.is_ok());
    }
}
