// src/utilities/validation/signup.rs

use crate::services::auth::signup::{SignupRequest, AuthProvider};
use crate::utilities::validation::email::{validate_email, EmailValidationError};
use crate::utilities::validation::password::{validate_password, PasswordValidationError};
use crate::utilities::validation::username::{validate_username, UsernameValidationError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignupValidationError {
    #[error("Email validation failed: {0}")]
    EmailError(#[from] EmailValidationError),
    #[error("Password validation failed: {0}")]
    PasswordError(#[from] PasswordValidationError),
    #[error("Username validation failed: {0}")]
    UsernameError(#[from] UsernameValidationError),
    #[error("Terms of service must be accepted")]
    TermsNotAccepted,
    #[error("Missing required fields for authentication provider")]
    MissingProviderFields,
}

pub fn validate_signup_request(request: &SignupRequest) -> Result<(), SignupValidationError> {
    log::debug!("Starting validation of signup request for email: {}", request.email);
    
    // Validate terms of service acceptance
    if !request.tos {
        log::warn!("Terms of service not accepted");
        return Err(SignupValidationError::TermsNotAccepted);
    }
    log::debug!("Terms of service validation passed");

    // Validate email
    log::debug!("Validating email: {}", request.email);
    if let Err(e) = validate_email(&request.email) {
        log::warn!("Email validation failed: {}", e);
        return Err(SignupValidationError::EmailError(e));
    }
    log::debug!("Email validation passed");

    // Validate username if provided
    if let Some(ref username) = request.username {
        log::debug!("Validating username: {}", username);
        if let Err(e) = validate_username(&request.username) {
            log::warn!("Username validation failed: {}", e);
            return Err(SignupValidationError::UsernameError(e));
        }
        log::debug!("Username validation passed");
    } else {
        log::debug!("No username provided, skipping username validation");
    }

    // Validate based on auth provider
    match request.provider {
        AuthProvider::Local => {
            log::debug!("Local authentication selected, validating password");
            // For local auth, password is required and must meet criteria
            if let Err(e) = validate_password(&request.password) {
                log::warn!("Password validation failed: {}", e);
                return Err(SignupValidationError::PasswordError(e));
            }
            log::debug!("Password validation passed");
        }
        AuthProvider::Google => {
            log::debug!("Google authentication selected, password validation skipped");
        }
        AuthProvider::GitHub => {
            log::debug!("GitHub authentication selected, password validation skipped");
        }
    }

    log::debug!("All signup validations passed successfully");
    Ok(())
}