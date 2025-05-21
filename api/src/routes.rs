// src/routes.rs

use actix_web::{web, Scope};
use crate::services::{hello_world, auth::signup};

// Configure API routes without version prefix
pub fn api_routes() -> Scope {
  web::scope("")
    .route("/hello", web::get().to(hello_world::hello))
    .route("/signup", web::post().to(signup::signup))
}

// Function to configure routes on the app
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(api_routes());
}
