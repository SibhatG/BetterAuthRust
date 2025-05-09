use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: Uuid,      // Subject (user ID)
    pub exp: usize,     // Expiration time (as UTC timestamp)
    pub iat: usize,     // Issued at (as UTC timestamp)
    pub is_admin: bool, // Is the user an admin
}

/// Create a JWT token with the given claims
pub fn create_jwt<T: Serialize>(claims: &T, secret: &str) -> Result<String, AuthError> {
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());
    
    encode(&Header::default(), claims, &encoding_key)
        .map_err(|e| AuthError::InternalServerError(format!("Failed to create JWT: {}", e)))
}

/// Decode and validate a JWT token with provided secret
pub fn decode_jwt_with_secret<T: for<'a> Deserialize<'a>>(token: &str, secret: &str) -> Result<T, AuthError> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::default();
    
    let token_data = decode::<T>(token, &decoding_key, &validation)
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            jsonwebtoken::errors::ErrorKind::InvalidToken => AuthError::InvalidToken,
            _ => AuthError::InvalidToken,
        })?;
    
    Ok(token_data.claims)
}

/// Decode and validate a JWT token using the application secret from config
pub fn decode_jwt<T: for<'a> Deserialize<'a>>(token: &str) -> Result<T, AuthError> {
    // In a real implementation, we would read the secret from a config
    // For now, we'll use a default development secret
    let secret = std::env::var("SECRET_KEY")
        .unwrap_or_else(|_| "development_secret_key_please_change_in_production".to_string());
    
    decode_jwt_with_secret(token, &secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_encode_decode() {
        let user_id = Uuid::new_v4();
        let secret = "test_secret_key";
        
        let claims = JwtClaims {
            sub: user_id,
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
            is_admin: false,
        };
        
        // Create token
        let token = create_jwt(&claims, secret).unwrap();
        
        // Decode token
        let decoded: JwtClaims = decode_jwt_with_secret(&token, secret).unwrap();
        
        assert_eq!(decoded.sub, user_id);
        assert_eq!(decoded.is_admin, false);
    }

    #[test]
    fn test_expired_token() {
        let user_id = Uuid::new_v4();
        let secret = "test_secret_key";
        
        let claims = JwtClaims {
            sub: user_id,
            exp: (Utc::now() - Duration::hours(1)).timestamp() as usize, // Expired 1 hour ago
            iat: (Utc::now() - Duration::hours(2)).timestamp() as usize,
            is_admin: false,
        };
        
        // Create token
        let token = create_jwt(&claims, secret).unwrap();
        
        // Decode token - should fail with TokenExpired
        let result: Result<JwtClaims, AuthError> = decode_jwt_with_secret(&token, secret);
        assert!(matches!(result, Err(AuthError::TokenExpired)));
    }
}
