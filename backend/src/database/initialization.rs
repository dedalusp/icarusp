use anyhow::Result;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env; // Import Result from anyhow

/// Establishes a direct database connection using Diesel
pub fn establish_connection() -> Result<PgConnection> {
    // Change return type to Result
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5433/diesel_demo".to_string());

    PgConnection::establish(&database_url)
        .map_err(|e| anyhow::anyhow!("Error connecting to {}: {}", database_url, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env; // Make sure env is imported here as well for clarity

    #[test]
    fn test_establish_connection() {
        // This test requires a valid database connection
        // Skip if DATABASE_URL is not set properly
        if env::var("DATABASE_URL").is_err() {
            println!("Skipping test: DATABASE_URL not set. Please create a .env file.");
            return;
        }

        // Print the DATABASE_URL that the application is actually using
        let current_db_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "DEFAULT_FALLBACK_URL_USED_IN_TEST".to_string());
        println!("DATABASE_URL being used by test: {}", current_db_url); // <--- ADD THIS LINE

        let connection = establish_connection();
        assert!(connection.is_ok());
        println!("Everything OKAY.");
    }
}
