use regex::Regex;
use lazy_static::lazy_static;

use crate::errors::AuthError;

lazy_static! {
    // Username regex: alphanumeric, underscores, hyphens, 3-30 chars
    static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_-]{3,30}$").unwrap();
    
    // Email regex: basic email validation
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
    ).unwrap();
    
    // Password regex: at least 8 chars, 1 uppercase, 1 lowercase, 1 number
    static ref PASSWORD_REGEX: Regex = Regex::new(
        r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d).{8,}$"
    ).unwrap();
}

/// Validate a username
pub fn validate_username(username: &str) -> Result<(), AuthError> {
    if !USERNAME_REGEX.is_match(username) {
        return Err(AuthError::ValidationError(
            "Username must be 3-30 characters and can only contain letters, numbers, underscores, and hyphens".into()
        ));
    }
    Ok(())
}

/// Validate an email address
pub fn validate_email(email: &str) -> Result<(), AuthError> {
    if !EMAIL_REGEX.is_match(email) {
        return Err(AuthError::ValidationError(
            "Invalid email format".into()
        ));
    }
    Ok(())
}

/// Validate a password
pub fn validate_password(password: &str) -> Result<(), AuthError> {
    if password.len() < 8 {
        return Err(AuthError::ValidationError(
            "Password must be at least 8 characters long".into()
        ));
    }

    if !PASSWORD_REGEX.is_match(password) {
        return Err(AuthError::ValidationError(
            "Password must contain at least one uppercase letter, one lowercase letter, and one number".into()
        ));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username() {
        // Valid usernames
        assert!(validate_username("user123").is_ok());
        assert!(validate_username("user_name").is_ok());
        assert!(validate_username("user-name").is_ok());
        
        // Invalid usernames
        assert!(validate_username("us").is_err()); // too short
        assert!(validate_username("user.name").is_err()); // invalid character
        assert!(validate_username("user@name").is_err()); // invalid character
    }

    #[test]
    fn test_validate_email() {
        // Valid emails
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("user.name@example.co.uk").is_ok());
        
        // Invalid emails
        assert!(validate_email("user@").is_err());
        assert!(validate_email("user@example").is_err());
        assert!(validate_email("user.example.com").is_err());
    }

    #[test]
    fn test_validate_password() {
        // Valid passwords
        assert!(validate_password("Password123").is_ok());
        assert!(validate_password("Secure_Password1").is_ok());
        
        // Invalid passwords
        assert!(validate_password("pass").is_err()); // too short
        assert!(validate_password("password").is_err()); // no uppercase or number
        assert!(validate_password("PASSWORD123").is_err()); // no lowercase
        assert!(validate_password("Password").is_err()); // no number
    }
}
