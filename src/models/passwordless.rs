use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::webauthn_simplified::{WebAuthnCredentialResponse, WebAuthnOptions};

/// Request to initiate passwordless registration
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PasswordlessRegisterStartRequest {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,
    
    #[validate(length(min = 1, message = "Email cannot be empty"))]
    #[validate(email(message = "Email must be valid"))]
    pub email: String,
    
    /// Optional device name for better identification
    pub device_name: Option<String>,
}

/// Response containing registration options
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordlessRegisterStartResponse {
    pub registration_id: String,
    pub options: WebAuthnOptions,
}

/// Request to complete passwordless registration
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordlessRegisterCompleteRequest {
    pub registration_id: String,
    pub credential: WebAuthnCredentialResponse,
}

/// Request to initiate passwordless login
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PasswordlessLoginStartRequest {
    #[validate(length(min = 1, message = "Username or email cannot be empty"))]
    pub username_or_email: String,
}

/// Response for starting passwordless login
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordlessLoginStartResponse {
    pub authentication_id: String,
    pub options: WebAuthnOptions,
}

/// Request to complete passwordless login
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordlessLoginCompleteRequest {
    pub authentication_id: String,
    pub credential: WebAuthnCredentialResponse,
}