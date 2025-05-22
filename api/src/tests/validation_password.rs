// api/src/tests/validation_password.rs
// Testing password validation logic

#[cfg(test)]
mod tests {
    use crate::utilities::validation::password::{validate_password, PasswordValidationError};

    #[test]
    fn test_valid_password() {
        assert!(validate_password(&Some("Password1!".to_string())).is_ok());
    }

    #[test]
    fn test_missing_password() {
        assert!(matches!(
            validate_password(&None),
            Err(PasswordValidationError::Missing)
        ));
    }

    #[test]
    fn test_too_short_password() {
        assert!(matches!(
            validate_password(&Some("Pwd1!".to_string())),
            Err(PasswordValidationError::TooShort { min: 8 })
        ));
    }

    #[test]
    fn test_no_uppercase() {
        assert!(matches!(
            validate_password(&Some("password1!".to_string())),
            Err(PasswordValidationError::NoUppercase)
        ));
    }

    #[test]
    fn test_no_lowercase() {
        assert!(matches!(
            validate_password(&Some("PASSWORD1!".to_string())),
            Err(PasswordValidationError::NoLowercase)
        ));
    }

    #[test]
    fn test_no_number() {
        assert!(matches!(
            validate_password(&Some("Password!".to_string())),
            Err(PasswordValidationError::NoNumber)
        ));
    }

    #[test]
    fn test_no_special_char() {
        assert!(matches!(
            validate_password(&Some("Password1".to_string())),
            Err(PasswordValidationError::NoSpecialChar)
        ));
    }
}