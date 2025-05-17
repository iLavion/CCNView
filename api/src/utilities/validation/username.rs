// src/utilities/validation/username.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UsernameValidationError {
    #[error("Username is too short (minimum {min} characters)")]
    TooShort { min: usize },
    #[error("Username is too long (maximum {max} characters)")]
    TooLong { max: usize },
    #[error("Username contains invalid characters (only alphanumeric, underscore, and hyphen are allowed)")]
    InvalidCharacters,
    #[error("Username must start with a letter")]
    InvalidStart,
    #[error("Username cannot end with underscore or hyphen")]
    InvalidEnd,
    #[error("Username cannot contain consecutive special characters")]
    ConsecutiveSpecialChars,
}

pub fn validate_username(username: &Option<String>) -> Result<(), UsernameValidationError> {
    // If username is None, it's optional so return Ok
    let username = match username {
        Some(username) => username,
        None => return Ok(()),
    };

    const MIN_USERNAME_LENGTH: usize = 3;
    const MAX_USERNAME_LENGTH: usize = 30;

    // Check username length
    if username.len() < MIN_USERNAME_LENGTH {
        return Err(UsernameValidationError::TooShort { min: MIN_USERNAME_LENGTH });
    }
    
    if username.len() > MAX_USERNAME_LENGTH {
        return Err(UsernameValidationError::TooLong { max: MAX_USERNAME_LENGTH });
    }

    // Check for valid characters (alphanumeric, underscore, hyphen)
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err(UsernameValidationError::InvalidCharacters);
    }

    // Check if username starts with a letter
    if !username.chars().next().unwrap().is_alphabetic() {
        return Err(UsernameValidationError::InvalidStart);
    }

    // Check if username ends with underscore or hyphen
    let last_char = username.chars().last().unwrap();
    if last_char == '_' || last_char == '-' {
        return Err(UsernameValidationError::InvalidEnd);
    }

    // Check for consecutive special characters
    let chars: Vec<char> = username.chars().collect();
    for i in 0..chars.len() - 1 {
        if (chars[i] == '_' || chars[i] == '-') && (chars[i + 1] == '_' || chars[i + 1] == '-') {
            return Err(UsernameValidationError::ConsecutiveSpecialChars);
        }
    }

    Ok(())
}