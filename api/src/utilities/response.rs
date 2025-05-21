// api/src/utilities/response.rs
//! # API Response Module

use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
  pub status: String,
  pub message: String,
  pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
  pub fn success(data: T) -> Self {
    ApiResponse {
      status: "ok".to_string(),
      message: "".to_string(),
      data: Some(data),
    }
  }

  pub fn error(message: &str) -> ApiResponse<T> {
    ApiResponse {
      status: "error".to_string(),
      message: message.to_string(),
      data: None,
    }
  }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ApiError {
  BadRequest(String),
  Unauthorized(String),
  Forbidden(String),
  NotFound(String),
  Conflict(String),
  InternalServerError(String),
}

impl fmt::Display for ApiError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let message = match self {
      ApiError::BadRequest(msg) => msg,
      ApiError::Unauthorized(msg) => msg,
      ApiError::Forbidden(msg) => msg,
      ApiError::NotFound(msg) => msg,
      ApiError::Conflict(msg) => msg,
      ApiError::InternalServerError(msg) => msg,
    };
    write!(f, "{}", message)
  }
}

impl ResponseError for ApiError {
  fn error_response(&self) -> HttpResponse {
    let error_response = ApiResponse::<()>::error(&self.to_string());
    match self {
      ApiError::BadRequest(_) => HttpResponse::BadRequest().json(error_response),
      ApiError::Unauthorized(_) => HttpResponse::Unauthorized().json(error_response),
      ApiError::Forbidden(_) => HttpResponse::Forbidden().json(error_response),
      ApiError::NotFound(_) => HttpResponse::NotFound().json(error_response),
      ApiError::Conflict(_) => HttpResponse::Conflict().json(error_response),
      ApiError::InternalServerError(_) => HttpResponse::InternalServerError().json(error_response),
    }
  }
}