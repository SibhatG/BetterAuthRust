use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::errors::AuthError;
use crate::models::{
    MfaRecoveryCode, NewMfaRecoveryCode, NewSession, NewUser, Session, User,
};

// In-memory database for testing/development
pub struct MemoryDb {
    users: Arc<Mutex<HashMap<Uuid, User>>>,
    sessions: Arc<Mutex<HashMap<Uuid, Session>>>,
    recovery_codes: Arc<Mutex<HashMap<Uuid, MfaRecoveryCode>>>,
}

impl MemoryDb {
    pub fn new() -> Self {
        MemoryDb {
            users: Arc::new(Mutex::new(HashMap::new())),
            sessions: Arc::new(Mutex::new(HashMap::new())),
            recovery_codes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // User methods
    pub async fn create_user(&self, user: NewUser) -> Result<User, AuthError> {
        let now = Utc::now();
        let user = User {
            id: user.id,
            username: user.username,
            email: user.email,
            password_hash: user.password_hash,
            is_email_verified: user.is_email_verified,
            email_verification_token: user.email_verification_token,
            email_verification_sent_at: user.email_verification_sent_at,
            password_reset_token: None,
            password_reset_sent_at: None,
            mfa_enabled: false,
            mfa_secret: None,
            created_at: now,
            updated_at: now,
            last_login_at: None,
            is_active: user.is_active,
            is_admin: user.is_admin,
        };

        {
            let mut users = self.users.lock().unwrap();
            users.insert(user.id, user.clone());
        }

        Ok(user)
    }

    pub async fn find_user_by_id(&self, id: Uuid) -> Result<User, AuthError> {
        let users = self.users.lock().unwrap();
        users
            .get(&id)
            .cloned()
            .ok_or(AuthError::UserNotFound)
    }

    pub async fn find_user_by_username(&self, username: &str) -> Result<User, AuthError> {
        let users = self.users.lock().unwrap();
        users
            .values()
            .find(|user| user.username == username)
            .cloned()
            .ok_or(AuthError::UserNotFound)
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<User, AuthError> {
        let users = self.users.lock().unwrap();
        users
            .values()
            .find(|user| user.email == email)
            .cloned()
            .ok_or(AuthError::UserNotFound)
    }

    pub async fn find_user_by_username_or_email(&self, username_or_email: &str) -> Result<User, AuthError> {
        let users = self.users.lock().unwrap();
        users
            .values()
            .find(|user| user.username == username_or_email || user.email == username_or_email)
            .cloned()
            .ok_or(AuthError::UserNotFound)
    }

    pub async fn find_user_by_verification_token(&self, token: &str) -> Result<User, AuthError> {
        let users = self.users.lock().unwrap();
        users
            .values()
            .find(|user| user.email_verification_token.as_deref() == Some(token))
            .cloned()
            .ok_or(AuthError::InvalidToken)
    }

    pub async fn find_user_by_reset_token(&self, token: &str) -> Result<User, AuthError> {
        let users = self.users.lock().unwrap();
        users
            .values()
            .find(|user| user.password_reset_token.as_deref() == Some(token))
            .cloned()
            .ok_or(AuthError::InvalidToken)
    }

    pub async fn user_exists_by_username(&self, username: &str) -> Result<bool, AuthError> {
        let users = self.users.lock().unwrap();
        Ok(users.values().any(|user| user.username == username))
    }

    pub async fn user_exists_by_email(&self, email: &str) -> Result<bool, AuthError> {
        let users = self.users.lock().unwrap();
        Ok(users.values().any(|user| user.email == email))
    }

    pub async fn update_last_login(&self, id: Uuid) -> Result<(), AuthError> {
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users.get_mut(&id) {
            user.last_login_at = Some(Utc::now());
            user.updated_at = Utc::now();
            Ok(())
        } else {
            Err(AuthError::UserNotFound)
        }
    }

    pub async fn update_verification_token(&self, id: Uuid, token: &str) -> Result<(), AuthError> {
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users.get_mut(&id) {
            user.email_verification_token = Some(token.to_string());
            user.email_verification_sent_at = Some(Utc::now());
            user.updated_at = Utc::now();
            Ok(())
        } else {
            Err(AuthError::UserNotFound)
        }
    }

    pub async fn update_password_reset_token(&self, id: Uuid, token: &str) -> Result<(), AuthError> {
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users.get_mut(&id) {
            user.password_reset_token = Some(token.to_string());
            user.password_reset_sent_at = Some(Utc::now());
            user.updated_at = Utc::now();
            Ok(())
        } else {
            Err(AuthError::UserNotFound)
        }
    }

    pub async fn update_password(&self, id: Uuid, password_hash: &str) -> Result<(), AuthError> {
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users.get_mut(&id) {
            user.password_hash = password_hash.to_string();
            user.password_reset_token = None;
            user.password_reset_sent_at = None;
            user.updated_at = Utc::now();
            Ok(())
        } else {
            Err(AuthError::UserNotFound)
        }
    }

    pub async fn verify_email(&self, id: Uuid) -> Result<User, AuthError> {
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users.get_mut(&id) {
            user.is_email_verified = true;
            user.email_verification_token = None;
            user.email_verification_sent_at = None;
            user.updated_at = Utc::now();
            Ok(user.clone())
        } else {
            Err(AuthError::UserNotFound)
        }
    }

    pub async fn update_mfa_secret(&self, id: Uuid, secret: &str) -> Result<(), AuthError> {
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users.get_mut(&id) {
            user.mfa_secret = Some(secret.to_string());
            user.updated_at = Utc::now();
            Ok(())
        } else {
            Err(AuthError::UserNotFound)
        }
    }

    pub async fn enable_mfa(&self, id: Uuid) -> Result<(), AuthError> {
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users.get_mut(&id) {
            user.mfa_enabled = true;
            user.updated_at = Utc::now();
            Ok(())
        } else {
            Err(AuthError::UserNotFound)
        }
    }

    pub async fn disable_mfa(&self, id: Uuid) -> Result<User, AuthError> {
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users.get_mut(&id) {
            user.mfa_enabled = false;
            user.mfa_secret = None;
            user.updated_at = Utc::now();
            Ok(user.clone())
        } else {
            Err(AuthError::UserNotFound)
        }
    }

    // Session methods
    pub async fn create_session(&self, session: NewSession) -> Result<Session, AuthError> {
        let now = Utc::now();
        let session = Session {
            id: session.id,
            user_id: session.user_id,
            refresh_token: session.refresh_token,
            user_agent: session.user_agent,
            ip_address: session.ip_address,
            expires_at: session.expires_at,
            created_at: now,
            updated_at: now,
            is_revoked: false,
        };

        {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.insert(session.id, session.clone());
        }

        Ok(session)
    }

    pub async fn find_session_by_id(&self, id: Uuid) -> Result<Session, AuthError> {
        let sessions = self.sessions.lock().unwrap();
        sessions
            .get(&id)
            .cloned()
            .ok_or(AuthError::InvalidToken)
    }

    pub async fn find_session_by_token(&self, token: &str) -> Result<Session, AuthError> {
        let sessions = self.sessions.lock().unwrap();
        sessions
            .values()
            .find(|session| session.refresh_token == token && !session.is_revoked)
            .cloned()
            .ok_or(AuthError::InvalidToken)
    }

    pub async fn find_sessions_by_user_id(&self, user_id: Uuid) -> Result<Vec<Session>, AuthError> {
        let sessions = self.sessions.lock().unwrap();
        let user_sessions: Vec<Session> = sessions
            .values()
            .filter(|session| session.user_id == user_id && !session.is_revoked)
            .cloned()
            .collect();

        Ok(user_sessions)
    }

    pub async fn revoke_session(&self, id: Uuid) -> Result<(), AuthError> {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get_mut(&id) {
            session.is_revoked = true;
            session.updated_at = Utc::now();
            Ok(())
        } else {
            Err(AuthError::InvalidToken)
        }
    }

    pub async fn revoke_all_sessions(&self, user_id: Uuid) -> Result<(), AuthError> {
        let mut sessions = self.sessions.lock().unwrap();
        for session in sessions.values_mut() {
            if session.user_id == user_id {
                session.is_revoked = true;
                session.updated_at = Utc::now();
            }
        }
        Ok(())
    }

    // MFA Recovery codes methods
    pub async fn create_recovery_code(&self, code: NewMfaRecoveryCode) -> Result<MfaRecoveryCode, AuthError> {
        let now = Utc::now();
        let recovery_code = MfaRecoveryCode {
            id: code.id,
            user_id: code.user_id,
            code: code.code,
            is_used: false,
            used_at: None,
            created_at: now,
        };

        {
            let mut codes = self.recovery_codes.lock().unwrap();
            codes.insert(recovery_code.id, recovery_code.clone());
        }

        Ok(recovery_code)
    }

    pub async fn use_recovery_code(&self, user_id: Uuid, code: &str) -> Result<bool, AuthError> {
        let mut codes = self.recovery_codes.lock().unwrap();
        if let Some(recovery_code) = codes
            .values_mut()
            .find(|rc| rc.user_id == user_id && rc.code == code && !rc.is_used)
        {
            recovery_code.is_used = true;
            recovery_code.used_at = Some(Utc::now());
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn delete_recovery_codes(&self, user_id: Uuid) -> Result<(), AuthError> {
        let mut codes = self.recovery_codes.lock().unwrap();
        codes.retain(|_, rc| rc.user_id != user_id);
        Ok(())
    }
}
