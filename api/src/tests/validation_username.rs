// api/src/tests/validation_username.rs
// Testing username validation logic

#[cfg(test)]
mod tests {
    use crate::utilities::validation::username::{validate_username, UsernameValidationError};

    #[test]
    fn test_valid_username() {
        assert!(validate_username(&Some("valid_user-name".to_string())).is_ok());
        assert!(validate_username(&Some("user123".to_string())).is_ok());
        assert!(validate_username(&None).is_ok());
    }

    #[test]
    fn test_too_short() {
        assert!(matches!(
            validate_username(&Some("us".to_string())),
            Err(UsernameValidationError::TooShort { min: 3 })
        ));
    }

    #[test]
    fn test_too_long() {
        let long_username = "a".repeat(31);
        assert!(matches!(
            validate_username(&Some(long_username)),
            Err(UsernameValidationError::TooLong { max: 30 })
        ));
    }

    #[test]
    fn test_invalid_characters() {
        assert!(matches!(
            validate_username(&Some("user@name".to_string())),
            Err(UsernameValidationError::InvalidCharacters)
        ));
    }

    #[test]
    fn test_invalid_start() {
        assert!(matches!(
            validate_username(&Some("_username".to_string())),
            Err(UsernameValidationError::InvalidStart)
        ));
        assert!(matches!(
            validate_username(&Some("123user".to_string())),
            Err(UsernameValidationError::InvalidStart)
        ));
    }

    #[test]
    fn test_invalid_end() {
        assert!(matches!(
            validate_username(&Some("username_".to_string())),
            Err(UsernameValidationError::InvalidEnd)
        ));
        assert!(matches!(
            validate_username(&Some("username-".to_string())),
            Err(UsernameValidationError::InvalidEnd)
        ));
    }

    #[test]
    fn test_consecutive_special_chars() {
        assert!(matches!(
            validate_username(&Some("user__name".to_string())),
            Err(UsernameValidationError::ConsecutiveSpecialChars)
        ));
        assert!(matches!(
            validate_username(&Some("user--name".to_string())),
            Err(UsernameValidationError::ConsecutiveSpecialChars)
        ));
        assert!(matches!(
            validate_username(&Some("user_-name".to_string())),
            Err(UsernameValidationError::ConsecutiveSpecialChars)
        ));
    }
}