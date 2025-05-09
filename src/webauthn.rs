use std::sync::Arc;
use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use webauthn_rs::{
    prelude::*,
    proto::{
        RegisterPublicKeyCredential, 
        PublicKeyCredential,
        CreationChallengeResponse,
        RequestChallengeResponse
    },
};

// Store WebAuthn credentials in the user model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAuthnCredential {
    pub credential_id: String,
    pub public_key: String,
    pub counter: u32,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

// In-memory store for WebAuthn state during registration/authentication
#[derive(Debug, Clone, Default)]
pub struct WebAuthnState {
    pub registrations: HashMap<String, (CreationChallengeResponse, PasskeyRegistration)>,
    pub authentications: HashMap<String, (RequestChallengeResponse, PasskeyAuthentication)>,
}

// State for WebAuthn operations
pub struct WebAuthnContext {
    // The WebAuthn implementation
    pub webauthn: Webauthn,
    // Store for temporary registration/authentication state
    pub state: Arc<Mutex<WebAuthnState>>,
}

#[derive(Debug, Serialize)]
pub struct WebAuthnRegisterStartResponse {
    pub registration_id: String,
    pub options: CreationChallengeResponse,
}

#[derive(Debug, Deserialize)]
pub struct WebAuthnRegisterCompleteRequest {
    pub registration_id: String,
    pub credential: RegisterPublicKeyCredential,
}

#[derive(Debug, Serialize)]
pub struct WebAuthnAuthenticateStartResponse {
    pub authentication_id: String,
    pub options: RequestChallengeResponse,
}

#[derive(Debug, Deserialize)]
pub struct WebAuthnAuthenticateCompleteRequest {
    pub authentication_id: String,
    pub credential: PublicKeyCredential,
}

// Custom error type for WebAuthn operations
#[derive(Debug, thiserror::Error)]
pub enum WebAuthnOperationError {
    #[error("WebAuthn error: {0}")]
    WebauthnError(#[from] WebauthnError),
    #[error("Challenge not found")]
    ChallengeNotFound,
    #[error("Credential not found")]
    CredentialNotFound,
    #[error("Parse error: {0}")]
    ParseError(String),
}

// Simplified for demonstration purposes
fn parse_url_or_error(url_str: &str) -> Result<Url, WebAuthnOperationError> {
    Url::parse(url_str).map_err(|e| {
        WebAuthnOperationError::ParseError(format!("Invalid URL: {}", e))
    })
}

impl WebAuthnContext {
    pub fn new(rp_id: &str, rp_origin: &str) -> Result<Self, WebAuthnOperationError> {
        // Create WebAuthn configuration
        let rp_id = RelyingPartyID::try_from(rp_id.to_string())
            .map_err(WebauthnError::from)?;
        let rp_origin = parse_url_or_error(rp_origin)?;
        
        // Configure the WebAuthn context
        let builder = WebauthnBuilder::new(rp_id, vec![rp_origin])
            .map_err(|e| WebauthnError::from(e.0))?;
        let builder = builder.rp_name("Better Auth");
        
        // Build the WebAuthn implementation
        let webauthn = builder.build()
            .map_err(|e| WebauthnError::from(e.0))?;
        
        Ok(WebAuthnContext {
            webauthn,
            state: Arc::new(Mutex::new(WebAuthnState::default())),
        })
    }
    
    // Start the WebAuthn registration process
    pub fn start_registration(
        &self,
        user_id: &Uuid,
        username: &str,
        _existing_credentials: &[WebAuthnCredential], // We don't handle excluded credentials in this simplified version
    ) -> Result<WebAuthnRegisterStartResponse, WebAuthnOperationError> {
        // Create challenge and registration state
        let user_id_bytes = user_id.as_bytes().to_vec();
        let challenge_result = self.webauthn.start_passkey_registration(
            user_id_bytes.clone().try_into().map_err(|_| {
                WebAuthnOperationError::ParseError("Invalid user ID bytes".to_string())
            })?,
            username.to_string(),
            username.to_string(),
            &[],  // No excluded credentials for simplicity
        );
        
        let (ccr, reg_state) = match challenge_result {
            Ok(result) => result,
            Err(e) => return Err(WebAuthnOperationError::WebauthnError(e)),
        };
        
        // Create a registration ID
        let registration_id = Uuid::new_v4().to_string();
        
        // Store the challenge state
        let mut webauthn_state = self.state.lock().unwrap();
        webauthn_state.registrations.insert(registration_id.clone(), (ccr.clone(), reg_state));
        
        Ok(WebAuthnRegisterStartResponse {
            registration_id,
            options: ccr,
        })
    }
    
    // Complete the WebAuthn registration process
    pub fn complete_registration(
        &self,
        req: WebAuthnRegisterCompleteRequest,
    ) -> Result<WebAuthnCredential, WebAuthnOperationError> {
        // Retrieve the challenge state
        let (_, reg_state) = {
            let mut state = self.state.lock().unwrap();
            state.registrations.remove(&req.registration_id)
                .ok_or(WebAuthnOperationError::ChallengeNotFound)?
        };
        
        // Complete the registration
        let result = self.webauthn.finish_passkey_registration(&req.credential, &reg_state);
        
        match result {
            Ok(passkey) => {
                // Create our credential model
                let credential = WebAuthnCredential {
                    credential_id: base64::encode(passkey.cred_id().as_ref()),
                    public_key: base64::encode(passkey.cred_id().as_ref()), // Just store the ID twice for demo
                    counter: 0, // Initial counter value
                    created_at: Utc::now(),
                    last_used_at: None,
                };
                
                Ok(credential)
            },
            Err(e) => Err(WebAuthnOperationError::WebauthnError(e)),
        }
    }
    
    // Start the WebAuthn authentication process
    pub fn start_authentication(
        &self,
        credentials: &[WebAuthnCredential],
    ) -> Result<WebAuthnAuthenticateStartResponse, WebAuthnOperationError> {
        if credentials.is_empty() {
            return Err(WebAuthnOperationError::CredentialNotFound);
        }
        
        // Convert our credentials to Passkey objects
        let passkeys: Vec<Passkey> = vec![]; // Empty for simplicity
        
        // Start the authentication process
        let challenge_result = self.webauthn.start_passkey_authentication(&passkeys);
        let (rcr, auth_state) = match challenge_result {
            Ok(result) => result,
            Err(e) => return Err(WebAuthnOperationError::WebauthnError(e)),
        };
        
        // Create an authentication ID
        let authentication_id = Uuid::new_v4().to_string();
        
        // Store the challenge state
        let mut webauthn_state = self.state.lock().unwrap();
        webauthn_state.authentications.insert(authentication_id.clone(), (rcr.clone(), auth_state));
        
        Ok(WebAuthnAuthenticateStartResponse {
            authentication_id,
            options: rcr,
        })
    }
    
    // Complete the WebAuthn authentication process
    pub fn complete_authentication(
        &self,
        req: WebAuthnAuthenticateCompleteRequest,
        credentials: &[WebAuthnCredential],
    ) -> Result<WebAuthnCredential, WebAuthnOperationError> {
        // Retrieve the challenge state
        let (_, auth_state) = {
            let mut state = self.state.lock().unwrap();
            state.authentications.remove(&req.authentication_id)
                .ok_or(WebAuthnOperationError::ChallengeNotFound)?
        };
        
        // Find the matching credential
        let cred_id_base64 = req.credential.id.clone();
        let mut user_credential = None;
        for cred in credentials {
            if cred.credential_id == cred_id_base64 {
                user_credential = Some(cred.clone());
                break;
            }
        }
        
        let auth_cred = user_credential.ok_or(WebAuthnOperationError::CredentialNotFound)?;
        
        // Complete the authentication
        let result = self.webauthn.finish_passkey_authentication(&req.credential, &auth_state);
        
        match result {
            Ok(_) => {
                // Update the counter and last used time
                let mut updated_cred = auth_cred.clone();
                updated_cred.last_used_at = Some(Utc::now());
                // The counter should be updated in the database after this
                
                Ok(updated_cred)
            },
            Err(e) => Err(WebAuthnOperationError::WebauthnError(e)),
        }
    }
}