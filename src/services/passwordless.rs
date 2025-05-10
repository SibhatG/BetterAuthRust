use uuid::Uuid;
use chrono::Utc;

use crate::models::{
    LoginResponse, PasswordlessLoginCompleteRequest, PasswordlessLoginStartRequest,
    PasswordlessLoginStartResponse, PasswordlessRegisterCompleteRequest,
    PasswordlessRegisterStartRequest, PasswordlessRegisterStartResponse, RegisterResponse,
};
use crate::errors::AuthError;
use crate::webauthn_simplified::{WebAuthnContext, WebAuthnCredential};
use crate::services::auth::AuthService;

impl AuthService {
    /// Start passwordless registration process
    pub async fn passwordless_register_start(
        &self,
        request: PasswordlessRegisterStartRequest,
    ) -> Result<PasswordlessRegisterStartResponse, AuthError> {
        // Check if username already exists
        if self.user_exists_by_username(&request.username).await? {
            return Err(AuthError::DuplicateUsername);
        }

        // Check if email already exists
        if self.user_exists_by_email(&request.email).await? {
            return Err(AuthError::DuplicateEmail);
        }

        // Create temporary user ID for WebAuthn
        let user_id = Uuid::new_v4();

        // Create WebAuthn context
        let webauthn_context = WebAuthnContext::new(&self.config.domain, &self.config.origin)?;

        // Start WebAuthn registration
        let webauthn_response = webauthn_context.start_registration(
            &user_id,
            &request.username,
            &[],
        )?;

        // Store registration data in server-side cache for later verification
        let cache_key = format!("passwordless_register:{}", &webauthn_response.registration_id);
        let cache_value = serde_json::json!({
            "user_id": user_id.to_string(),
            "username": request.username,
            "email": request.email,
            "device_name": request.device_name,
            "registration_id": webauthn_response.registration_id,
            "challenge": webauthn_response.options.challenge,
            "timestamp": Utc::now().to_rfc3339(),
        });

        // Cache the registration data with expiration (15 minutes)
        let cache_ttl = self.config.cache_ttl_seconds.unwrap_or(900);
        self.cache.set_ex(&cache_key, &cache_value.to_string(), cache_ttl).await?;

        // Return registration options to client
        Ok(PasswordlessRegisterStartResponse {
            registration_id: webauthn_response.registration_id,
            options: webauthn_response.options,
        })
    }

    /// Complete passwordless registration
    pub async fn passwordless_register_complete(
        &self,
        request: PasswordlessRegisterCompleteRequest,
        ip: Option<String>,
        user_agent: Option<String>,
    ) -> Result<RegisterResponse, AuthError> {
        // Get registration data from cache
        let cache_key = format!("passwordless_register:{}", &request.registration_id);
        let cached_data = self.cache.get(&cache_key).await?;
        
        // Check if registration data exists
        let cached_data = match cached_data {
            Some(data) => data,
            None => return Err(AuthError::InvalidToken("Registration expired".to_string())),
        };

        // Parse cached data
        let cached_json: serde_json::Value = serde_json::from_str(&cached_data)
            .map_err(|_| AuthError::ServerError("Failed to parse cached data".to_string()))?;

        // Extract data from cache
        let user_id = Uuid::parse_str(cached_json["user_id"].as_str().unwrap_or_default())
            .map_err(|_| AuthError::ServerError("Invalid user ID".to_string()))?;
        let username = cached_json["username"].as_str()
            .ok_or_else(|| AuthError::ServerError("Missing username".to_string()))?.to_string();
        let email = cached_json["email"].as_str()
            .ok_or_else(|| AuthError::ServerError("Missing email".to_string()))?.to_string();
        let device_name = cached_json["device_name"].as_str().map(|s| s.to_string());

        // Create WebAuthn context
        let webauthn_context = WebAuthnContext::new(&self.config.domain, &self.config.origin)?;

        // Complete WebAuthn registration
        let credential = webauthn_context.complete_registration(request)?;

        // Create user with passwordless credential
        let user = self.create_passwordless_user(
            user_id,
            &username,
            &email,
            vec![credential],
            ip,
            user_agent,
        ).await?;

        // Delete cache entry
        self.cache.del(&cache_key).await?;

        // Generate tokens
        let (access_token, refresh_token) = self.generate_tokens(user.id, None).await?;

        // Return success response
        Ok(RegisterResponse {
            user: user.into(),
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.config.access_token_expiry_seconds,
        })
    }

    /// Start passwordless login process
    pub async fn passwordless_login_start(
        &self,
        request: PasswordlessLoginStartRequest,
    ) -> Result<PasswordlessLoginStartResponse, AuthError> {
        // Find user by username or email
        let user = self.find_user_by_username_or_email(&request.username_or_email).await?;

        // Get user's WebAuthn credentials
        let user_credentials = self.get_user_webauthn_credentials(user.id).await?;
        
        // Ensure user has WebAuthn credentials
        if user_credentials.is_empty() {
            return Err(AuthError::Unauthorized("No passwordless credentials".to_string()));
        }

        // Create WebAuthn context
        let webauthn_context = WebAuthnContext::new(&self.config.domain, &self.config.origin)?;

        // Start WebAuthn authentication
        let webauthn_response = webauthn_context.start_authentication(&user_credentials)?;

        // Store authentication data in server-side cache for later verification
        let cache_key = format!("passwordless_login:{}", &webauthn_response.authentication_id);
        let cache_value = serde_json::json!({
            "user_id": user.id.to_string(),
            "username_or_email": request.username_or_email,
            "authentication_id": webauthn_response.authentication_id,
            "challenge": webauthn_response.options.challenge,
            "timestamp": Utc::now().to_rfc3339(),
        });

        // Cache the authentication data with expiration (15 minutes)
        let cache_ttl = self.config.cache_ttl_seconds.unwrap_or(900);
        self.cache.set_ex(&cache_key, &cache_value.to_string(), cache_ttl).await?;

        // Return authentication options to client
        Ok(PasswordlessLoginStartResponse {
            authentication_id: webauthn_response.authentication_id,
            options: webauthn_response.options,
        })
    }

    /// Complete passwordless login
    pub async fn passwordless_login_complete(
        &self,
        request: PasswordlessLoginCompleteRequest,
        ip: Option<String>,
        user_agent: Option<String>,
    ) -> Result<LoginResponse, AuthError> {
        // Get authentication data from cache
        let cache_key = format!("passwordless_login:{}", &request.authentication_id);
        let cached_data = self.cache.get(&cache_key).await?;
        
        // Check if authentication data exists
        let cached_data = match cached_data {
            Some(data) => data,
            None => return Err(AuthError::InvalidToken("Authentication expired".to_string())),
        };

        // Parse cached data
        let cached_json: serde_json::Value = serde_json::from_str(&cached_data)
            .map_err(|_| AuthError::ServerError("Failed to parse cached data".to_string()))?;

        // Extract user ID from cache
        let user_id = Uuid::parse_str(cached_json["user_id"].as_str().unwrap_or_default())
            .map_err(|_| AuthError::ServerError("Invalid user ID".to_string()))?;

        // Get user and credentials
        let user = self.get_user_by_id(user_id).await?;
        let user_credentials = self.get_user_webauthn_credentials(user_id).await?;

        // Create WebAuthn context
        let webauthn_context = WebAuthnContext::new(&self.config.domain, &self.config.origin)?;

        // Complete WebAuthn authentication
        let updated_credential = webauthn_context.complete_authentication(
            request,
            &user_credentials,
        )?;

        // Update the credential's counter and last used timestamp
        self.update_webauthn_credential(user_id, updated_credential).await?;

        // Generate login tokens
        let (access_token, refresh_token) = self.generate_tokens(user_id, None).await?;

        // Create a new session
        let session_id = self.create_session(
            user_id, 
            &access_token, 
            ip, 
            user_agent,
            true, // Passwordless authentication is considered verified
        ).await?;

        // Delete cache entry
        self.cache.del(&cache_key).await?;

        // Return login response
        Ok(LoginResponse {
            user: user.into(),
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.config.access_token_expiry_seconds,
            session_id,
        })
    }

    /// Create a user with passwordless credentials
    async fn create_passwordless_user(
        &self,
        user_id: Uuid,
        username: &str,
        email: &str,
        credentials: Vec<WebAuthnCredential>,
        ip: Option<String>,
        user_agent: Option<String>,
    ) -> Result<crate::models::User, AuthError> {
        // Start database transaction
        let mut tx = self.pool.begin().await?;

        // Create user in database (use a random password since passwordless login doesn't need a password)
        let random_password = Uuid::new_v4().to_string();
        let hashed_password = self.hash_password(&random_password)?;

        // Create user with email verified (since WebAuthn authenticator verifies presence)
        let user = sqlx::query_as!(
            crate::models::User,
            r#"
            INSERT INTO users (id, username, email, password, email_verified, created_at, updated_at)
            VALUES ($1, $2, $3, $4, true, NOW(), NOW())
            RETURNING id, username, email, password, email_verified, mfa_enabled, 
                      created_at, updated_at, last_login_at, account_locked, 
                      account_locked_reason as "account_locked_reason: String", password_expires_at
            "#,
            user_id,
            username,
            email,
            hashed_password,
        )
        .fetch_one(&mut *tx)
        .await?;

        // Store each WebAuthn credential in the database
        for credential in credentials {
            sqlx::query!(
                r#"
                INSERT INTO webauthn_credentials
                (user_id, credential_id, public_key, counter, created_at, last_used_at, device_name)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
                user_id,
                credential.credential_id,
                credential.public_key,
                credential.counter,
                credential.created_at,
                credential.last_used_at,
                None::<String>, // Device name not supported in this current implementation
            )
            .execute(&mut *tx)
            .await?;
        }

        // Commit the transaction
        tx.commit().await?;

        // Return the created user
        Ok(user)
    }

    /// Get user's WebAuthn credentials from the database
    async fn get_user_webauthn_credentials(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<WebAuthnCredential>, AuthError> {
        // Query the database for the user's WebAuthn credentials
        let credentials = sqlx::query_as!(
            WebAuthnCredential,
            r#"
            SELECT
                credential_id,
                public_key,
                counter,
                created_at,
                last_used_at
            FROM
                webauthn_credentials
            WHERE
                user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(credentials)
    }

    /// Update a WebAuthn credential in the database
    async fn update_webauthn_credential(
        &self,
        user_id: Uuid,
        credential: WebAuthnCredential,
    ) -> Result<(), AuthError> {
        // Update the credential in the database
        sqlx::query!(
            r#"
            UPDATE webauthn_credentials
            SET counter = $3, last_used_at = $4
            WHERE user_id = $1 AND credential_id = $2
            "#,
            user_id,
            credential.credential_id,
            credential.counter,
            credential.last_used_at,
        )
        .execute(&self.pool)
        .await?;

        // Also update the user's last login timestamp
        sqlx::query!(
            r#"
            UPDATE users
            SET last_login_at = NOW()
            WHERE id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}