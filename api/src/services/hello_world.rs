// src/services/hello_world.rs

use actix_web::HttpResponse;
use crate::models::utilities::response::{Response, Status};

// GET - Hello world data
pub async fn hello() -> HttpResponse {

  let response = Response::new(
    Status::Ok,
    "Hello world retrieved successfully".to_string(),
    "Hello world".to_string(),
  );
  
  HttpResponse::Ok().json(response)
}