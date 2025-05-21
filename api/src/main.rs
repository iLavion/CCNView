// api/src/main.rs
// Entry point for the API

use actix_web::{App, HttpServer};
use api::routes::configure_routes;
use api::configs::database::get_db_client;
use api::health::health_check;
use api::utilities::logger::setup_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Environment variables
    dotenvy::dotenv().ok();

    // Logger
    setup_logger();
    log::info!("{}", "=".repeat(48));
    log::info!("Initializing API Server...");

    // Health checks
    if let Err(e) = health_check().await {
        eprintln!("{e}");
        std::process::exit(1);
    }

    // Database
    let db_client = get_db_client().await;

    // Server
    log::info!("API has been initialized at http://127.0.0.1:8080");
    log::info!("{}", "=".repeat(48));

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db_client.clone()))
            .configure(configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}