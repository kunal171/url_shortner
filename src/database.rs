use mongodb::{Client, Database};
use std::error::Error;

pub async fn connect_db() -> Result<Database, Box<dyn Error + Send + Sync>> {
    // Load .env variables if .env file exists.
    dotenvy::dotenv().ok();

    // If this fails, return the error to the caller.
    let db_uri = std::env::var("MONGODB_URI")?;

    // If DATABASE_NAME is missing, use a default value.
    let db_name = std::env::var("DATABASE_NAME")
        .unwrap_or_else(|_| "url_shortner".to_string());

    // If MongoDB connection fails, return the error to the caller.
    let client = Client::with_uri_str(&db_uri).await?;

    // Return selected database handle.
    Ok(client.database(&db_name))
}