// src/utilities/validation/email.rs

use log::{debug, info, warn};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmailValidationError {
    #[error("Email cannot be empty")]
    Empty,
    #[error("Email format is invalid")]
    InvalidFormat,
    #[error("Email domain is not allowed")]
    InvalidDomain,
    #[error("Email is too long (maximum {max} characters)")]
    TooLong { max: usize },
}

pub fn validate_email(email: &str) -> Result<(), EmailValidationError> {
    debug!("Validating email: {}", email);
    
    // Check if email is empty
    if email.is_empty() {
        warn!("Email validation failed: Email is empty");
        return Err(EmailValidationError::Empty);
    }

    // Check email length (typical DB field limit)
    const MAX_EMAIL_LENGTH: usize = 254;
    if email.len() > MAX_EMAIL_LENGTH {
        warn!("Email validation failed: Email is too long ({} characters, max {})",
            email.len(), MAX_EMAIL_LENGTH);
        return Err(EmailValidationError::TooLong { max: MAX_EMAIL_LENGTH });
    }

    // Basic email format validation
    // Check for @ symbol and at least one character before and after
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        warn!("Email validation failed: Invalid format (missing @ or empty parts)");
        return Err(EmailValidationError::InvalidFormat);
    }

    // Domain should have at least one dot
    let domain_parts: Vec<&str> = parts[1].split('.').collect();
    if domain_parts.len() < 2 || domain_parts.iter().any(|part| part.is_empty()) {
        warn!("Email validation failed: Invalid domain format");
        return Err(EmailValidationError::InvalidFormat);
    }

    info!("Email validation successful for: {}", email);
    Ok(())
}