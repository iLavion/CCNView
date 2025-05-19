// src/services/auth/signup.rs

use actix_web::{web, HttpResponse, HttpRequest};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use log::{info, warn, error, debug};

use crate::models::users::user::User;
use crate::repositories::user::{UserRepository, RepositoryError};
use crate::configs::database::get_db_client;
use crate::utilities::{
    hashing::password::{hash_password, PasswordError},
    response::{ApiResponse, ApiError},
    validation::signup::{validate_signup_request, SignupValidationError},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuthProvider {
    Local,
    Google,
    GitHub,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Password error: {0}")]
    PasswordError(#[from] PasswordError),
    #[error("Repository error: {0}")]
    RepositoryError(#[from] RepositoryError),
    #[error("Validation error: {0}")]
    ValidationError(#[from] SignupValidationError),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub provider: AuthProvider,
    pub tos: bool,
    pub newsletter: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupResponse {
    pub user_id: String,
    pub email: String,
    pub message: String,
}

pub async fn signup(
    req: HttpRequest,
    body: web::Json<SignupRequest>,
) -> Result<HttpResponse, ApiError> {
    // Log the raw request information
    info!("Received signup request from: {}", req.peer_addr().unwrap_or_else(|| std::net::SocketAddr::from(([0, 0, 0, 0], 0))));
    info!("Content-Type: {:?}", req.headers().get("content-type"));
    info!("Request path: {}", req.path());
    debug!("Request body received: {:?}", body);

    info!("Received signup request for email: {}, auth provider: {:?}", 
          body.email, body.provider);
    
    // Validate the signup request using our validation utilities
    info!("Validating signup request...");
    if let Err(err) = validate_signup_request(&body) {
        warn!("Signup validation failed: {}", err);
        return Err(map_validation_error_to_api_error(err));
    }
    info!("Signup request validation successful");

    // Get MongoDB client
    info!("Connecting to database...");
    let db_client = get_db_client().await;
    let user_repo = UserRepository::new(db_client);

    // Hash password if provided (local auth)
    let password_hash = if body.provider == AuthProvider::Local {
        info!("Hashing password for local auth user...");
        match &body.password {
            Some(password) => {
                match hash_password(password.as_bytes()) {
                    Ok((hash, _salt)) => {
                        debug!("Password hashed successfully");
                        Some(hash)
                    }
                    Err(e) => {
                        error!("Password hashing failed: {}", e);
                        return Err(ApiError::InternalServerError(e.to_string()));
                    }
                }
            }
            None => {
                warn!("Password missing for local auth provider");
                return Err(ApiError::BadRequest("Password is required for local authentication".to_string()));
            }
        }
    } else {
        debug!("No password hashing needed for OAuth provider: {:?}", body.provider);
        None
    };

    // Create user
    info!("Creating new user with email: {}", body.email);
    let user = User::new(
        body.email.clone(),
        body.username.clone(),
        password_hash,
        body.provider.clone(),
        body.newsletter,
        body.tos,
    );
    debug!("User object created: {:?}", user);

    // Attempt to create the user
    info!("Saving user to database...");
    let user_id = match user_repo.create_user(user).await {
        Ok(id) => {
            info!("User created successfully with ID: {}", id);
            id
        }
        Err(RepositoryError::DuplicateEmail) => {
            warn!("Signup failed: Email {} is already registered", body.email);
            return Err(ApiError::Conflict("Email is already registered".to_string()));
        }
        Err(RepositoryError::DuplicateUsername) => {
            warn!("Signup failed: Username {:?} is already taken", body.username);
            return Err(ApiError::Conflict("Username is already taken".to_string()));
        }
        Err(e) => {
            error!("Failed to create user: {}", e);
            return Err(ApiError::InternalServerError(format!("Failed to create user: {}", e)));
        }
    };

    // Return successful response
    info!("Signup successful for user ID: {}", user_id);
    let response = SignupResponse {
        user_id: user_id.to_string(),
        email: body.email.clone(),
        message: "User registered successfully. Please check your email for verification.".to_string(),
    };

    Ok(HttpResponse::Created().json(ApiResponse::success(response)))
}

// Helper function to map validation errors to API errors
fn map_validation_error_to_api_error(err: SignupValidationError) -> ApiError {
    match err {
        SignupValidationError::EmailError(e) => ApiError::BadRequest(format!("Email error: {}", e)),
        SignupValidationError::PasswordError(e) => ApiError::BadRequest(format!("{}", e)),
        SignupValidationError::UsernameError(e) => ApiError::BadRequest(format!("{}", e)),
        SignupValidationError::TermsNotAccepted => ApiError::BadRequest("Terms of service must be accepted".to_string()),
        SignupValidationError::MissingProviderFields => ApiError::BadRequest("Missing required fields for authentication provider".to_string()),
    }
}