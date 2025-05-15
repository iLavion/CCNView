// src/configs/server.rs

#[allow(dead_code)]
pub struct ServerConfig {
  pub host: String,
  pub port: u16,
}

#[allow(dead_code)]
impl ServerConfig {
  pub fn new() -> Self {
    ServerConfig {
      host: "127.0.0.1".to_string(),
      port: 5174,
    }
  }

  pub fn from_env() -> Self {
    ServerConfig {
      host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
      port: std::env::var("SERVER_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080),
    }
  }
}

#[allow(dead_code)]
pub struct OAuthConfig {
  pub google_client_id: String,
  pub google_client_secret: String,
}

#[allow(dead_code)]
impl OAuthConfig {
  pub fn new() -> Self {
    OAuthConfig {
      google_client_id: std::env::var("GOOGLE_CLIENT_ID")
        .unwrap_or_else(|_| "default_google_client_id".to_string()),
      google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET")
        .unwrap_or_else(|_| "default_google_client_secret".to_string()),
    }
  }

  pub fn from_env() -> Self {
    OAuthConfig {
      google_client_id: std::env::var("GOOGLE_CLIENT_ID").unwrap_or_else(|_| "default_google_client_id".to_string()),
      google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET").unwrap_or_else(|_| "default_google_client_secret".to_string()),
    }
  }
}