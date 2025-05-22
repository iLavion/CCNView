// api/src/tests/validation_email.rs
// Testing email validation logic

#[cfg(test)]
mod tests {
    use crate::utilities::validation::email::{validate_email, EmailValidationError};

    #[test]
    fn test_valid_email() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("user.name+tag@example.co.uk").is_ok());
    }

    #[test]
    fn test_empty_email() {
        assert!(matches!(
            validate_email(""),
            Err(EmailValidationError::Empty)
        ));
    }

    #[test]
    fn test_invalid_format() {
        assert!(matches!(
            validate_email("test@"),
            Err(EmailValidationError::InvalidFormat)
        ));
        assert!(matches!(
            validate_email("@example.com"),
            Err(EmailValidationError::InvalidFormat)
        ));
        assert!(matches!(
            validate_email("test@example"),
            Err(EmailValidationError::InvalidFormat)
        ));
    }

    #[test]
    fn test_too_long() {
        let long_email = format!("{}@example.com", "a".repeat(250));
        assert!(matches!(
            validate_email(&long_email),
            Err(EmailValidationError::TooLong { max: 254 })
        ));
    }
}