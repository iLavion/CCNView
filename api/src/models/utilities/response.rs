// src/models/utilities/response.rs
use serde::{Deserialize, Serialize};

// Status enum for responses
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
  Ok,
  Error,
}

// Generic response structure
#[allow(dead_code)]
#[derive(Serialize)]
pub struct Response<T> {
  pub data: T,
  pub status: Status,
  pub message: String,
}

// Implementation of response struct
#[allow(dead_code)]
impl<T> Response<T> {
  pub fn new(status: Status, message: String, data: T) -> Self {
    Response {
      status,
      message,
      data,
    }
  }
}