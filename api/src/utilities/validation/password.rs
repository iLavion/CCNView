// src/utilities/validation/password.rs

use log::{debug, info, warn};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordValidationError {
    #[error("Password is required")]
    Missing,
    #[error("Password is too short (minimum {min} characters)")]
    TooShort { min: usize },
    #[error("Password is too long (maximum {max} characters)")]
    TooLong { max: usize },
    #[error("Password must contain at least one uppercase letter")]
    NoUppercase,
    #[error("Password must contain at least one lowercase letter")]
    NoLowercase,
    #[error("Password must contain at least one number")]
    NoNumber,
    #[error("Password must contain at least one special character")]
    NoSpecialChar,
}

pub fn validate_password(password: &Option<String>) -> Result<(), PasswordValidationError> {
    debug!("Starting password validation");
    
    // Check if password is provided
    let password = match password {
        Some(password) => password,
        None => {
            warn!("Password is required but not provided");
            return Err(PasswordValidationError::Missing);
        }
    };

    const MIN_PASSWORD_LENGTH: usize = 8;
    const MAX_PASSWORD_LENGTH: usize = 100;

    // Check password length
    if password.len() < MIN_PASSWORD_LENGTH {
        warn!("Password is too short ({} chars, minimum {})",
            password.len(), MIN_PASSWORD_LENGTH);
        return Err(PasswordValidationError::TooShort { min: MIN_PASSWORD_LENGTH });
    }
    
    if password.len() > MAX_PASSWORD_LENGTH {
        warn!("Password is too long ({} chars, maximum {})",
            password.len(), MAX_PASSWORD_LENGTH);
        return Err(PasswordValidationError::TooLong { max: MAX_PASSWORD_LENGTH });
    }

    // Check for uppercase letters
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        warn!("Password lacks uppercase characters");
        return Err(PasswordValidationError::NoUppercase);
    }

    // Check for lowercase letters
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        warn!("Password lacks lowercase characters");
        return Err(PasswordValidationError::NoLowercase);
    }

    // Check for numbers
    if !password.chars().any(|c| c.is_ascii_digit()) {
        warn!("Password lacks numeric characters");
        return Err(PasswordValidationError::NoNumber);
    }

    // Check for special characters (non-alphanumeric)
    if !password.chars().any(|c| !c.is_alphanumeric()) {
        warn!("Password validation failed: Password lacks special characters");
        return Err(PasswordValidationError::NoSpecialChar);
    }

    info!("Password validation successful");
    Ok(())
}