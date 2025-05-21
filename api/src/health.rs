// api/src/health.rs
// Health check module for the API

use crate::configs::database::init_db_client;

/// Runs startup health checks (e.g., database connectivity).
pub async fn health_check() -> Result<(), String> {
    log::info!("{}", "=".repeat(48));
    log::info!("Running health checks...");
    
    // Check database connection
    match init_db_client().await {
        Ok(_) => {
            log::info!("✅ Database connection established successfully");
        },
        Err(e) => {
            log::error!("❌ Database connection failed: {e}");
            return Err(format!("Database connection failed: {e}"));
        }
    }
    
    // Add more health checks here as needed
    
    // Only log this after all checks have passed
    log::info!("All health checks passed.");
    log::info!("{}", "=".repeat(48));
    Ok(())
}
