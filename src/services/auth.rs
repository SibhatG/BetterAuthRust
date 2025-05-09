use std::sync::Arc;

use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::db::DatabaseConnection;
use crate::errors::AuthError;
use crate::models::{
    DisableMfaRequest, EnableMfaRequest, LoginRequest, LogoutRequest, LoginResponse,
    MfaLoginRequest, MfaRecoveryCodesResponse, MfaRecoveryRequest, MfaSetupResponse,
    MfaVerifyRequest, MfaVerifyResponse, NewMfaRecoveryCode, NewSession, NewUser,
    PasswordResetConfirmRequest, PasswordResetRequest, PasswordResetResponse, RefreshTokenRequest,
    RefreshTokenResponse, RegisterRequest, RegisterResponse, Session, SessionResponse,
    User, UserResponse, VerifyEmailRequest,
};
use crate::services::email::EmailService;
use crate::services::mfa::MfaService;
use crate::utils::{
    jwt::{create_jwt, JwtClaims},
    password::hash_password, password::verify_password,
    validation::validate_email, validation::validate_password, validation::validate_username,
};
use crate::config::Config;

pub struct AuthService {
    db: Arc<DatabaseConnection>,
    email_service: EmailService,
    mfa_service: MfaService,
    config: Config,
}

impl AuthService {
    pub fn new(db: Arc<DatabaseConnection>, config: Config) -> Self {
        let email_service = EmailService::new(config.clone());
        let mfa_service = MfaService::new();
        
        AuthService {
            db,
            email_service,
            mfa_service,
            config,
        }
    }

    pub async fn register(
        &self,
        data: RegisterRequest,
        ip: Option<String>,
        user_agent: Option<String>,
    ) -> Result<RegisterResponse, AuthError> {
        // Validate input
        validate_username(&data.username)?;
        validate_email(&data.email)?;
        validate_password(&data.password)?;

        if data.password != data.password_confirmation {
            return Err(AuthError::ValidationError("Passwords do not match".into()));
        }

        // Check if user already exists
        if self.db.user_exists_by_username(&data.username).await? {
            return Err(AuthError::UsernameExists);
        }

        if self.db.user_exists_by_email(&data.email).await? {
            return Err(AuthError::EmailExists);
        }

        // Hash password
        let password_hash = hash_password(&data.password)?;

        // Generate verification token
        let verification_token = Uuid::new_v4().to_string();

        // Create user
        let new_user = NewUser {
            id: Uuid::new_v4(),
            username: data.username,
            email: data.email.clone(),
            password_hash,
            is_email_verified: false,
            email_verification_token: Some(verification_token.clone()),
            email_verification_sent_at: Some(Utc::now()),
            is_active: true,
            is_admin: false,
        };

        let user = self.db.create_user(new_user).await?;

        // Send verification email
        self.email_service
            .send_verification_email(&user.email, &verification_token)
            .await?;

        Ok(RegisterResponse {
            user: user.into(),
            message: "User registered successfully. Please verify your email.".into(),
        })
    }

    pub async fn login(
        &self,
        data: LoginRequest,
        ip: Option<String>,
        user_agent: Option<String>,
    ) -> Result<LoginResponse, AuthError> {
        // Find user by username or email
        let user = self
            .db
            .find_user_by_username_or_email(&data.username_or_email)
            .await?;

        // Verify password
        if !verify_password(&data.password, &user.password_hash)? {
            return Err(AuthError::InvalidCredentials);
        }

        // Check if user is active
        if !user.is_active {
            return Err(AuthError::PermissionDenied);
        }

        // Check if MFA is required
        if user.mfa_enabled {
            // Return partial response with MFA required flag
            return Ok(LoginResponse {
                access_token: String::new(),
                refresh_token: String::new(),
                token_type: "Bearer".into(),
                expires_in: 0,
                user: user.clone().into(),
                mfa_required: true,
            });
        }

        // Generate tokens
        let access_token = self.create_access_token(&user)?;
        let refresh_token = Uuid::new_v4().to_string();

        // Save refresh token
        let expires_at = Utc::now() + Duration::seconds(self.config.jwt.refresh_token_expiry as i64);
        let session = NewSession {
            id: Uuid::new_v4(),
            user_id: user.id,
            refresh_token: refresh_token.clone(),
            user_agent,
            ip_address: ip,
            expires_at,
        };

        self.db.create_session(session).await?;

        // Update last login
        self.db.update_last_login(user.id).await?;

        Ok(LoginResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".into(),
            expires_in: self.config.jwt.access_token_expiry,
            user: user.into(),
            mfa_required: false,
        })
    }

    pub async fn mfa_login(
        &self,
        data: MfaLoginRequest,
        ip: Option<String>,
        user_agent: Option<String>,
    ) -> Result<LoginResponse, AuthError> {
        // Find user by username or email
        let user = self
            .db
            .find_user_by_username_or_email(&data.username_or_email)
            .await?;

        // Verify password
        if !verify_password(&data.password, &user.password_hash)? {
            return Err(AuthError::InvalidCredentials);
        }

        // Check if user is active
        if !user.is_active {
            return Err(AuthError::PermissionDenied);
        }

        // Check if MFA is enabled
        if !user.mfa_enabled {
            return Err(AuthError::ValidationError("MFA is not enabled for this user".into()));
        }

        // Check if MFA code or recovery code is provided
        if data.mfa_code.is_none() && data.recovery_code.is_none() {
            return Err(AuthError::MfaRequired);
        }

        // Verify MFA
        if let Some(mfa_code) = data.mfa_code {
            // Verify MFA code
            if let Some(secret) = &user.mfa_secret {
                if !self.mfa_service.verify_totp(secret, &mfa_code) {
                    return Err(AuthError::InvalidMfaCode);
                }
            } else {
                return Err(AuthError::InternalServerError("MFA secret not found".into()));
            }
        } else if let Some(recovery_code) = data.recovery_code {
            // Verify recovery code
            let used = self.db.use_recovery_code(user.id, &recovery_code).await?;
            if !used {
                return Err(AuthError::InvalidMfaCode);
            }
        }

        // Generate tokens
        let access_token = self.create_access_token(&user)?;
        let refresh_token = Uuid::new_v4().to_string();

        // Save refresh token
        let expires_at = Utc::now() + Duration::seconds(self.config.jwt.refresh_token_expiry as i64);
        let session = NewSession {
            id: Uuid::new_v4(),
            user_id: user.id,
            refresh_token: refresh_token.clone(),
            user_agent,
            ip_address: ip,
            expires_at,
        };

        self.db.create_session(session).await?;

        // Update last login
        self.db.update_last_login(user.id).await?;

        Ok(LoginResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".into(),
            expires_in: self.config.jwt.access_token_expiry,
            user: user.into(),
            mfa_required: false,
        })
    }

    pub async fn refresh_token(
        &self,
        data: RefreshTokenRequest,
        ip: Option<String>,
        user_agent: Option<String>,
    ) -> Result<RefreshTokenResponse, AuthError> {
        // Find session by refresh token
        let session = self.db.find_session_by_token(&data.refresh_token).await?;

        // Check if session is valid
        if session.is_revoked || session.expires_at < Utc::now() {
            return Err(AuthError::InvalidToken);
        }

        // Find user
        let user = self.db.find_user_by_id(session.user_id).await?;

        // Check if user is active
        if !user.is_active {
            return Err(AuthError::PermissionDenied);
        }

        // Generate new tokens
        let access_token = self.create_access_token(&user)?;
        let refresh_token = Uuid::new_v4().to_string();

        // Revoke old session
        self.db.revoke_session(session.id).await?;

        // Save new refresh token
        let expires_at = Utc::now() + Duration::seconds(self.config.jwt.refresh_token_expiry as i64);
        let new_session = NewSession {
            id: Uuid::new_v4(),
            user_id: user.id,
            refresh_token: refresh_token.clone(),
            user_agent,
            ip_address: ip,
            expires_at,
        };

        self.db.create_session(new_session).await?;

        Ok(RefreshTokenResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".into(),
            expires_in: self.config.jwt.access_token_expiry,
        })
    }

    pub async fn logout(
        &self,
        data: LogoutRequest,
        user_id: Option<Uuid>,
    ) -> Result<LogoutResponse, AuthError> {
        // If refresh token is provided, revoke the specific session
        if let Some(refresh_token) = data.refresh_token {
            let session = self.db.find_session_by_token(&refresh_token).await?;

            // Check if user_id matches (if authenticated)
            if let Some(uid) = user_id {
                if session.user_id != uid {
                    return Err(AuthError::PermissionDenied);
                }
            }

            self.db.revoke_session(session.id).await?;
        }

        Ok(LogoutResponse {
            message: "Logged out successfully".into(),
        })
    }

    pub async fn logout_all(
        &self,
        user_id: Uuid,
    ) -> Result<LogoutResponse, AuthError> {
        self.db.revoke_all_sessions(user_id).await?;

        Ok(LogoutResponse {
            message: "All sessions logged out successfully".into(),
        })
    }

    pub async fn verify_email(
        &self,
        data: VerifyEmailRequest,
    ) -> Result<UserResponse, AuthError> {
        // Find user by verification token
        let user = self
            .db
            .find_user_by_verification_token(&data.token)
            .await?;

        // Verify email
        let user = self.db.verify_email(user.id).await?;

        Ok(user.into())
    }

    pub async fn resend_verification_email(
        &self,
        user_id: Uuid,
    ) -> Result<PasswordResetResponse, AuthError> {
        // Find user
        let user = self.db.find_user_by_id(user_id).await?;

        // Check if email is already verified
        if user.is_email_verified {
            return Err(AuthError::ValidationError("Email is already verified".into()));
        }

        // Generate new verification token
        let verification_token = Uuid::new_v4().to_string();

        // Update user
        self.db
            .update_verification_token(user.id, &verification_token)
            .await?;

        // Send verification email
        self.email_service
            .send_verification_email(&user.email, &verification_token)
            .await?;

        Ok(PasswordResetResponse {
            message: "Verification email sent successfully".into(),
        })
    }

    pub async fn password_reset_request(
        &self,
        data: PasswordResetRequest,
    ) -> Result<PasswordResetResponse, AuthError> {
        // Find user by email
        let user = match self.db.find_user_by_email(&data.email).await {
            Ok(user) => user,
            Err(_) => {
                // Return success even if user doesn't exist for security reasons
                return Ok(PasswordResetResponse {
                    message: "If the email is registered, a password reset link has been sent".into(),
                });
            }
        };

        // Generate reset token
        let reset_token = Uuid::new_v4().to_string();

        // Update user
        self.db
            .update_password_reset_token(user.id, &reset_token)
            .await?;

        // Send password reset email
        self.email_service
            .send_password_reset_email(&user.email, &reset_token)
            .await?;

        Ok(PasswordResetResponse {
            message: "If the email is registered, a password reset link has been sent".into(),
        })
    }

    pub async fn password_reset_confirm(
        &self,
        data: PasswordResetConfirmRequest,
    ) -> Result<PasswordResetResponse, AuthError> {
        // Validate password
        validate_password(&data.password)?;

        if data.password != data.password_confirmation {
            return Err(AuthError::ValidationError("Passwords do not match".into()));
        }

        // Find user by reset token
        let user = self
            .db
            .find_user_by_reset_token(&data.token)
            .await?;

        // Check if token is expired (24 hours)
        if let Some(sent_at) = user.password_reset_sent_at {
            if (Utc::now() - sent_at).num_seconds() > 86400 {
                return Err(AuthError::TokenExpired);
            }
        } else {
            return Err(AuthError::InvalidToken);
        }

        // Hash new password
        let password_hash = hash_password(&data.password)?;

        // Update password and clear reset token
        self.db
            .update_password(user.id, &password_hash)
            .await?;

        // Revoke all sessions
        self.db.revoke_all_sessions(user.id).await?;

        Ok(PasswordResetResponse {
            message: "Password updated successfully".into(),
        })
    }

    pub async fn mfa_setup(&self, user_id: Uuid) -> Result<MfaSetupResponse, AuthError> {
        // Find user
        let user = self.db.find_user_by_id(user_id).await?;

        // Check if MFA is already enabled
        if user.mfa_enabled {
            return Err(AuthError::ValidationError("MFA is already enabled".into()));
        }

        // Generate TOTP secret
        let (secret, qr_code_url) = self
            .mfa_service
            .generate_totp_secret(&user.username);

        // Save secret temporarily
        self.db.update_mfa_secret(user.id, &secret).await?;

        Ok(MfaSetupResponse {
            secret,
            qr_code_url,
        })
    }

    pub async fn mfa_enable(
        &self,
        user_id: Uuid,
        data: EnableMfaRequest,
    ) -> Result<MfaRecoveryCodesResponse, AuthError> {
        // Find user
        let user = self.db.find_user_by_id(user_id).await?;

        // Check if MFA is already enabled
        if user.mfa_enabled {
            return Err(AuthError::ValidationError("MFA is already enabled".into()));
        }

        // Check if secret exists
        if user.mfa_secret.is_none() {
            return Err(AuthError::ValidationError("MFA setup not initiated".into()));
        }

        // Verify TOTP code
        let secret = user.mfa_secret.unwrap();
        if !self.mfa_service.verify_totp(&secret, &data.mfa_code) {
            return Err(AuthError::InvalidMfaCode);
        }

        // Enable MFA
        self.db.enable_mfa(user.id).await?;

        // Generate recovery codes
        let recovery_codes = self.generate_recovery_codes(user.id).await?;

        Ok(MfaRecoveryCodesResponse { recovery_codes })
    }

    pub async fn mfa_disable(
        &self,
        user_id: Uuid,
        data: DisableMfaRequest,
    ) -> Result<UserResponse, AuthError> {
        // Find user
        let user = self.db.find_user_by_id(user_id).await?;

        // Check if MFA is enabled
        if !user.mfa_enabled {
            return Err(AuthError::ValidationError("MFA is not enabled".into()));
        }

        // Verify password
        if !verify_password(&data.password, &user.password_hash)? {
            return Err(AuthError::InvalidCredentials);
        }

        // Verify TOTP code
        if let Some(secret) = &user.mfa_secret {
            if !self.mfa_service.verify_totp(secret, &data.mfa_code) {
                return Err(AuthError::InvalidMfaCode);
            }
        } else {
            return Err(AuthError::InternalServerError("MFA secret not found".into()));
        }

        // Disable MFA
        let user = self.db.disable_mfa(user.id).await?;

        // Delete recovery codes
        self.db.delete_recovery_codes(user.id).await?;

        Ok(user.into())
    }

    pub async fn mfa_verify(
        &self,
        data: VerifyMfaRequest,
        ip: Option<String>,
        user_agent: Option<String>,
    ) -> Result<MfaVerifyResponse, AuthError> {
        // MFA verification happens after login, so we should have a temporary session
        // This implementation is simplified - you'd need a temporary token system
        
        // This is a placeholder for demonstration
        // In a real system, you'd verify the temporary token, get the user ID
        // For now, we'll just use a hardcoded example
        
        return Err(AuthError::InternalServerError("MFA verification not fully implemented".into()));
        
        // The rest of the function would look something like:
        /*
        // Verify the temporary token to get the user
        let user = verify_temp_token(...)?;
        
        // Verify TOTP code
        if let Some(secret) = &user.mfa_secret {
            if !self.mfa_service.verify_totp(secret, &data.mfa_code) {
                return Err(AuthError::InvalidMfaCode);
            }
        } else {
            return Err(AuthError::InternalServerError("MFA secret not found".into()));
        }

        // Generate tokens
        let access_token = self.create_access_token(&user)?;
        let refresh_token = Uuid::new_v4().to_string();

        // Save refresh token
        let expires_at = Utc::now() + Duration::seconds(self.config.jwt.refresh_token_expiry as i64);
        let session = NewSession {
            id: Uuid::new_v4(),
            user_id: user.id,
            refresh_token: refresh_token.clone(),
            user_agent,
            ip_address: ip,
            expires_at,
        };

        self.db.create_session(session).await?;

        // Update last login
        self.db.update_last_login(user.id).await?;

        Ok(MfaVerifyResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".into(),
            expires_in: self.config.jwt.access_token_expiry,
            user: user.into(),
        })
        */
    }

    pub async fn mfa_recovery(
        &self,
        data: MfaRecoveryRequest,
        ip: Option<String>,
        user_agent: Option<String>,
    ) -> Result<MfaVerifyResponse, AuthError> {
        // Similar to mfa_verify, this requires a temporary token system
        // This is a placeholder
        
        return Err(AuthError::InternalServerError("MFA recovery not fully implemented".into()));
    }

    pub async fn mfa_recovery_codes(
        &self,
        user_id: Uuid,
    ) -> Result<MfaRecoveryCodesResponse, AuthError> {
        // Find user
        let user = self.db.find_user_by_id(user_id).await?;

        // Check if MFA is enabled
        if !user.mfa_enabled {
            return Err(AuthError::ValidationError("MFA is not enabled".into()));
        }

        // Generate new recovery codes
        self.db.delete_recovery_codes(user.id).await?;
        let recovery_codes = self.generate_recovery_codes(user.id).await?;

        Ok(MfaRecoveryCodesResponse { recovery_codes })
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<UserResponse, AuthError> {
        let user = self.db.find_user_by_id(user_id).await?;
        Ok(user.into())
    }

    pub async fn get_sessions(&self, user_id: Uuid) -> Result<Vec<SessionResponse>, AuthError> {
        let sessions = self.db.find_sessions_by_user_id(user_id).await?;
        
        let response: Vec<SessionResponse> = sessions
            .into_iter()
            .map(|s| {
                let mut sr: SessionResponse = s.into();
                // Mark current session - this requires comparing with the actual session token
                // which we don't have here without a context
                sr.is_current = false;
                sr
            })
            .collect();
        
        Ok(response)
    }

    pub async fn revoke_session(
        &self,
        user_id: Uuid,
        session_id: Uuid,
    ) -> Result<LogoutResponse, AuthError> {
        // Find session
        let session = self.db.find_session_by_id(session_id).await?;

        // Check if user owns the session
        if session.user_id != user_id {
            return Err(AuthError::PermissionDenied);
        }

        // Revoke session
        self.db.revoke_session(session.id).await?;

        Ok(LogoutResponse {
            message: "Session revoked successfully".into(),
        })
    }

    // Helper functions

    fn create_access_token(&self, user: &User) -> Result<String, AuthError> {
        let claims = JwtClaims {
            sub: user.id,
            exp: (Utc::now() + Duration::seconds(self.config.jwt.access_token_expiry as i64)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
            is_admin: user.is_admin,
        };

        create_jwt(&claims, &self.config.jwt.secret)
    }

    async fn generate_recovery_codes(&self, user_id: Uuid) -> Result<Vec<String>, AuthError> {
        let mut codes = Vec::new();

        // Generate 10 recovery codes
        for _ in 0..10 {
            let code = self.mfa_service.generate_recovery_code();
            codes.push(code.clone());

            // Save recovery code
            let recovery_code = NewMfaRecoveryCode {
                id: Uuid::new_v4(),
                user_id,
                code,
            };

            self.db.create_recovery_code(recovery_code).await?;
        }

        Ok(codes)
    }
}
