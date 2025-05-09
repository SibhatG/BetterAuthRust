use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Email already exists")]
    EmailExists,
    
    #[error("Username already exists")]
    UsernameExists,
    
    #[error("Invalid token")]
    InvalidToken,
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("Email not verified")]
    EmailNotVerified,
    
    #[error("Invalid verification code")]
    InvalidVerificationCode,
    
    #[error("MFA required")]
    MfaRequired,
    
    #[error("Invalid MFA code")]
    InvalidMfaCode,
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Permission denied")]
    PermissionDenied,
    
    #[error("Email error: {0}")]
    EmailError(String),
    
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl AuthError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidCredentials | Self::InvalidToken | Self::TokenExpired | Self::InvalidMfaCode | Self::InvalidVerificationCode => {
                StatusCode::UNAUTHORIZED
            }
            Self::UserNotFound => StatusCode::NOT_FOUND,
            Self::EmailExists | Self::UsernameExists | Self::ValidationError(_) => {
                StatusCode::BAD_REQUEST
            }
            Self::MfaRequired | Self::EmailNotVerified => StatusCode::FORBIDDEN,
            Self::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
            Self::PermissionDenied => StatusCode::FORBIDDEN,
            Self::DatabaseError(_) | Self::EmailError(_) | Self::InternalServerError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
    status_code: u16,
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        self.status_code()
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            error: self.error_type(),
            message: self.to_string(),
            status_code: status_code.as_u16(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

impl AuthError {
    fn error_type(&self) -> String {
        match self {
            Self::InvalidCredentials => "INVALID_CREDENTIALS",
            Self::UserNotFound => "USER_NOT_FOUND",
            Self::EmailExists => "EMAIL_EXISTS",
            Self::UsernameExists => "USERNAME_EXISTS",
            Self::InvalidToken => "INVALID_TOKEN",
            Self::TokenExpired => "TOKEN_EXPIRED",
            Self::EmailNotVerified => "EMAIL_NOT_VERIFIED",
            Self::InvalidVerificationCode => "INVALID_VERIFICATION_CODE",
            Self::MfaRequired => "MFA_REQUIRED",
            Self::InvalidMfaCode => "INVALID_MFA_CODE",
            Self::DatabaseError(_) => "DATABASE_ERROR",
            Self::ValidationError(_) => "VALIDATION_ERROR",
            Self::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
            Self::PermissionDenied => "PERMISSION_DENIED",
            Self::EmailError(_) => "EMAIL_ERROR",
            Self::InternalServerError(_) => "INTERNAL_SERVER_ERROR",
        }
        .to_string()
    }
}

impl From<diesel::result::Error> for AuthError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => AuthError::UserNotFound,
            _ => AuthError::DatabaseError(err.to_string()),
        }
    }
}

impl From<argon2::Error> for AuthError {
    fn from(err: argon2::Error) -> Self {
        AuthError::InternalServerError(format!("Password hashing error: {}", err))
    }
}

impl From<jsonwebtoken::errors::Error> for AuthError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            jsonwebtoken::errors::ErrorKind::InvalidToken => AuthError::InvalidToken,
            _ => AuthError::InvalidToken,
        }
    }
}

impl From<validator::ValidationErrors> for AuthError {
    fn from(err: validator::ValidationErrors) -> Self {
        AuthError::ValidationError(err.to_string())
    }
}

impl From<lettre::error::Error> for AuthError {
    fn from(err: lettre::error::Error) -> Self {
        AuthError::EmailError(err.to_string())
    }
}

impl From<std::io::Error> for AuthError {
    fn from(err: std::io::Error) -> Self {
        AuthError::InternalServerError(err.to_string())
    }
}
