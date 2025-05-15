// src/configs/database.rs

use mongodb::{Client, Database, options::ClientOptions};
use std::env;
use std::sync::OnceLock;

static DB_CLIENT: OnceLock<Client> = OnceLock::new();

// Get database name
fn get_database_name() -> String {
    env::var("MONGODB_DATABASE").unwrap_or_else(|_| "CCNViewDB".to_string())
}

// Initialize MongoDB client
pub async fn init_db_client() -> Result<&'static Client, mongodb::error::Error> {
    if let Some(client) = DB_CLIENT.get() {
        return Ok(client);
    }

    // Get connection details from environment variables
    let host = std::env::var("MONGODB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("MONGODB_PORT").unwrap_or_else(|_| "27784".to_string());
    let db_name = std::env::var("MONGODB_DATABASE").unwrap_or_else(|_| "CCNViewDB".to_string());

    // Simple connection string without authentication
    let connection_string = format!("mongodb://{}:{}", host, port);
    log::debug!(
        "Connecting to MongoDB with simple connection string: {}",
        connection_string
    );

    let mut client_options = ClientOptions::parse(connection_string).await?;

    // Connection pool
    client_options.max_pool_size = Some(10);
    client_options.min_pool_size = Some(2);

    log::debug!("Connecting to MongoDB at {}:{}/{}", host, port, db_name);

    // Create client
    let client = Client::with_options(client_options)?;

    // Try to ping to verify connection
    match client
        .database("admin")
        .run_command(mongodb::bson::doc! { "ping": 1 })
        .await
    {
        Ok(_) => {
            log::debug!("Successfully connected to MongoDB");

            // Test database access
            match client.database(&db_name).list_collection_names().await {
                Ok(collections) => {
                    log::debug!(
                        "Database '{}' accessible. Found {} collections",
                        db_name,
                        collections.len()
                    );
                    log::debug!("Collections in database: {:?}", collections);
                }
                Err(e) => {
                    log::error!("Cannot access database '{}': {}", db_name, e);
                    return Err(e);
                }
            }
        }
        Err(e) => {
            log::error!("Failed to ping MongoDB: {}", e);
            return Err(e);
        }
    }

    // Store the client in the static cell
    match DB_CLIENT.set(client) {
        Ok(_) => Ok(DB_CLIENT.get().unwrap()),
        Err(_) => Ok(DB_CLIENT.get().unwrap()),
    }
}

// Get the MongoDB client (initializes if needed)
pub async fn get_db_client() -> &'static Client {
    if let Some(client) = DB_CLIENT.get() {
        return client;
    }
    // Initialize client if it doesn't exist
    let client = init_db_client()
        .await
        .expect("Failed to initialize database connection");
    log::info!("Database connection established");
    client
}

// Get a specific database instance with authentication
#[allow(dead_code)]
pub async fn get_database() -> Database {
    let client = get_db_client().await;
    let db_name = get_database_name();

    log::info!("Using database: {}", db_name);
    client.database(&db_name)
}

// Get a specific collection with proper authentication
#[allow(dead_code)]
pub async fn get_collection(collection_name: &str) -> mongodb::Collection<mongodb::bson::Document> {
    let db = get_database().await;
    log::info!("Accessing collection: {}", collection_name);
    db.collection(collection_name)
}
