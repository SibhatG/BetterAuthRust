use diesel::{
    dsl::now,
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::errors::AuthError;
use crate::models::{
    MfaRecoveryCode, NewMfaRecoveryCode, NewSession, NewUser, Session, User,
};
use crate::schema::{mfa_recovery_codes, sessions, users};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgConn = PooledConnection<ConnectionManager<PgConnection>>;

pub struct PostgresDb {
    pool: PgPool,
}

impl PostgresDb {
    pub fn new(pool: PgPool) -> Self {
        PostgresDb { pool }
    }

    fn get_conn(&self) -> Result<PgConn, AuthError> {
        self.pool.get().map_err(|e| {
            AuthError::DatabaseError(format!("Failed to get database connection: {}", e))
        })
    }

    // User methods
    pub async fn create_user(&self, user: NewUser) -> Result<User, AuthError> {
        use diesel::insert_into;

        let conn = self.get_conn()?;
        
        let user = tokio::task::spawn_blocking(move || {
            insert_into(users::table)
                .values(&user)
                .get_result::<User>(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::DatabaseError(format!("Insert user error: {}", e)))?;
        
        Ok(user)
    }

    pub async fn find_user_by_id(&self, id: Uuid) -> Result<User, AuthError> {
        let conn = self.get_conn()?;
        
        let user = tokio::task::spawn_blocking(move || {
            users::table
                .find(id)
                .first::<User>(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::from(e))?;
        
        Ok(user)
    }

    pub async fn find_user_by_username(&self, username: &str) -> Result<User, AuthError> {
        let username = username.to_string();
        let conn = self.get_conn()?;
        
        let user = tokio::task::spawn_blocking(move || {
            users::table
                .filter(users::username.eq(username))
                .first::<User>(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::from(e))?;
        
        Ok(user)
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<User, AuthError> {
        let email = email.to_string();
        let conn = self.get_conn()?;
        
        let user = tokio::task::spawn_blocking(move || {
            users::table
                .filter(users::email.eq(email))
                .first::<User>(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::from(e))?;
        
        Ok(user)
    }

    pub async fn find_user_by_username_or_email(&self, username_or_email: &str) -> Result<User, AuthError> {
        let username_or_email = username_or_email.to_string();
        let conn = self.get_conn()?;
        
        let user = tokio::task::spawn_blocking(move || {
            users::table
                .filter(
                    users::username.eq(&username_or_email).or(
                        users::email.eq(&username_or_email)
                    )
                )
                .first::<User>(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::from(e))?;
        
        Ok(user)
    }

    pub async fn find_user_by_verification_token(&self, token: &str) -> Result<User, AuthError> {
        let token = token.to_string();
        let conn = self.get_conn()?;
        
        let user = tokio::task::spawn_blocking(move || {
            users::table
                .filter(users::email_verification_token.eq(token))
                .first::<User>(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|_| AuthError::InvalidToken)?;
        
        Ok(user)
    }

    pub async fn find_user_by_reset_token(&self, token: &str) -> Result<User, AuthError> {
        let token = token.to_string();
        let conn = self.get_conn()?;
        
        let user = tokio::task::spawn_blocking(move || {
            users::table
                .filter(users::password_reset_token.eq(token))
                .first::<User>(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|_| AuthError::InvalidToken)?;
        
        Ok(user)
    }

    pub async fn user_exists_by_username(&self, username: &str) -> Result<bool, AuthError> {
        let username = username.to_string();
        let conn = self.get_conn()?;
        
        let exists = tokio::task::spawn_blocking(move || {
            diesel::select(diesel::dsl::exists(
                users::table.filter(users::username.eq(username))
            ))
            .get_result::<bool>(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::DatabaseError(format!("Query error: {}", e)))?;
        
        Ok(exists)
    }

    pub async fn user_exists_by_email(&self, email: &str) -> Result<bool, AuthError> {
        let email = email.to_string();
        let conn = self.get_conn()?;
        
        let exists = tokio::task::spawn_blocking(move || {
            diesel::select(diesel::dsl::exists(
                users::table.filter(users::email.eq(email))
            ))
            .get_result::<bool>(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::DatabaseError(format!("Query error: {}", e)))?;
        
        Ok(exists)
    }

    pub async fn update_last_login(&self, id: Uuid) -> Result<(), AuthError> {
        let conn = self.get_conn()?;
        
        tokio::task::spawn_blocking(move || {
            diesel::update(users::table.find(id))
                .set((
                    users::last_login_at.eq(now),
                    users::updated_at.eq(now),
                ))
                .execute(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::DatabaseError(format!("Update error: {}", e)))?;
        
        Ok(())
    }

    pub async fn update_verification_token(&self, id: Uuid, token: &str) -> Result<(), AuthError> {
        let token = token.to_string();
        let conn = self.get_conn()?;
        
        tokio::task::spawn_blocking(move || {
            diesel::update(users::table.find(id))
                .set((
                    users::email_verification_token.eq(token),
                    users::email_verification_sent_at.eq(now),
                    users::updated_at.eq(now),
                ))
                .execute(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::DatabaseError(format!("Update error: {}", e)))?;
        
        Ok(())
    }

    pub async fn update_password_reset_token(&self, id: Uuid, token: &str) -> Result<(), AuthError> {
        let token = token.to_string();
        let conn = self.get_conn()?;
        
        tokio::task::spawn_blocking(move || {
            diesel::update(users::table.find(id))
                .set((
                    users::password_reset_token.eq(token),
                    users::password_reset_sent_at.eq(now),
                    users::updated_at.eq(now),
                ))
                .execute(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::DatabaseError(format!("Update error: {}", e)))?;
        
        Ok(())
    }

    pub async fn update_password(&self, id: Uuid, password_hash: &str) -> Result<(), AuthError> {
        let password_hash = password_hash.to_string();
        let conn = self.get_conn()?;
        
        tokio::task::spawn_blocking(move || {
            diesel::update(users::table.find(id))
                .set((
                    users::password_hash.eq(password_hash),
                    users::password_reset_token.eq::<Option<String>>(None),
                    users::password_reset_sent_at.eq::<Option<DateTime<Utc>>>(None),
                    users::updated_at.eq(now),
                ))
                .execute(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::DatabaseError(format!("Update error: {}", e)))?;
        
        Ok(())
    }

    pub async fn verify_email(&self, id: Uuid) -> Result<User, AuthError> {
        let conn = self.get_conn()?;
        
        let user = tokio::task::spawn_blocking(move || {
            diesel::update(users::table.find(id))
                .set((
                    users::is_email_verified.eq(true),
                    users::email_verification_token.eq::<Option<String>>(None),
                    users::email_verification_sent_at.eq::<Option<DateTime<Utc>>>(None),
                    users::updated_at.eq(now),
                ))
                .get_result::<User>(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::DatabaseError(format!("Update error: {}", e)))?;
        
        Ok(user)
    }

    pub async fn update_mfa_secret(&self, id: Uuid, secret: &str) -> Result<(), AuthError> {
        let secret = secret.to_string();
        let conn = self.get_conn()?;
        
        tokio::task::spawn_blocking(move || {
            diesel::update(users::table.find(id))
                .set((
                    users::mfa_secret.eq(secret),
                    users::updated_at.eq(now),
                ))
                .execute(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::DatabaseError(format!("Update error: {}", e)))?;
        
        Ok(())
    }

    pub async fn enable_mfa(&self, id: Uuid) -> Result<(), AuthError> {
        let conn = self.get_conn()?;
        
        tokio::task::spawn_blocking(move || {
            diesel::update(users::table.find(id))
                .set((
                    users::mfa_enabled.eq(true),
                    users::updated_at.eq(now),
                ))
                .execute(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::DatabaseError(format!("Update error: {}", e)))?;
        
        Ok(())
    }

    pub async fn disable_mfa(&self, id: Uuid) -> Result<User, AuthError> {
        let conn = self.get_conn()?;
        
        let user = tokio::task::spawn_blocking(move || {
            diesel::update(users::table.find(id))
                .set((
                    users::mfa_enabled.eq(false),
                    users::mfa_secret.eq::<Option<String>>(None),
                    users::updated_at.eq(now),
                ))
                .get_result::<User>(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::DatabaseError(format!("Update error: {}", e)))?;
        
        Ok(user)
    }

    // Session methods
    pub async fn create_session(&self, session: NewSession) -> Result<Session, AuthError> {
        let conn = self.get_conn()?;
        
        let session = tokio::task::spawn_blocking(move || {
            diesel::insert_into(sessions::table)
                .values(&session)
                .get_result::<Session>(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::DatabaseError(format!("Insert error: {}", e)))?;
        
        Ok(session)
    }

    pub async fn find_session_by_id(&self, id: Uuid) -> Result<Session, AuthError> {
        let conn = self.get_conn()?;
        
        let session = tokio::task::spawn_blocking(move || {
            sessions::table
                .find(id)
                .first::<Session>(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|_| AuthError::InvalidToken)?;
        
        Ok(session)
    }

    pub async fn find_session_by_token(&self, token: &str) -> Result<Session, AuthError> {
        let token = token.to_string();
        let conn = self.get_conn()?;
        
        let session = tokio::task::spawn_blocking(move || {
            sessions::table
                .filter(sessions::refresh_token.eq(token))
                .filter(sessions::is_revoked.eq(false))
                .first::<Session>(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|_| AuthError::InvalidToken)?;
        
        Ok(session)
    }

    pub async fn find_sessions_by_user_id(&self, user_id: Uuid) -> Result<Vec<Session>, AuthError> {
        let conn = self.get_conn()?;
        
        let sessions = tokio::task::spawn_blocking(move || {
            sessions::table
                .filter(sessions::user_id.eq(user_id))
                .filter(sessions::is_revoked.eq(false))
                .order(sessions::created_at.desc())
                .load::<Session>(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::DatabaseError(format!("Query error: {}", e)))?;
        
        Ok(sessions)
    }

    pub async fn revoke_session(&self, id: Uuid) -> Result<(), AuthError> {
        let conn = self.get_conn()?;
        
        tokio::task::spawn_blocking(move || {
            diesel::update(sessions::table.find(id))
                .set((
                    sessions::is_revoked.eq(true),
                    sessions::updated_at.eq(now),
                ))
                .execute(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::DatabaseError(format!("Update error: {}", e)))?;
        
        Ok(())
    }

    pub async fn revoke_all_sessions(&self, user_id: Uuid) -> Result<(), AuthError> {
        let conn = self.get_conn()?;
        
        tokio::task::spawn_blocking(move || {
            diesel::update(sessions::table.filter(sessions::user_id.eq(user_id)))
                .set((
                    sessions::is_revoked.eq(true),
                    sessions::updated_at.eq(now),
                ))
                .execute(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::DatabaseError(format!("Update error: {}", e)))?;
        
        Ok(())
    }

    // MFA Recovery codes methods
    pub async fn create_recovery_code(&self, code: NewMfaRecoveryCode) -> Result<MfaRecoveryCode, AuthError> {
        let conn = self.get_conn()?;
        
        let recovery_code = tokio::task::spawn_blocking(move || {
            diesel::insert_into(mfa_recovery_codes::table)
                .values(&code)
                .get_result::<MfaRecoveryCode>(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::DatabaseError(format!("Insert error: {}", e)))?;
        
        Ok(recovery_code)
    }

    pub async fn use_recovery_code(&self, user_id: Uuid, code: &str) -> Result<bool, AuthError> {
        let code = code.to_string();
        let conn = self.get_conn()?;
        
        let result = tokio::task::spawn_blocking(move || {
            conn.transaction(|| {
                let recovery_code = mfa_recovery_codes::table
                    .filter(mfa_recovery_codes::user_id.eq(user_id))
                    .filter(mfa_recovery_codes::code.eq(&code))
                    .filter(mfa_recovery_codes::is_used.eq(false))
                    .first::<MfaRecoveryCode>(&conn)
                    .optional()?;
                
                if let Some(recovery_code) = recovery_code {
                    diesel::update(mfa_recovery_codes::table.find(recovery_code.id))
                        .set((
                            mfa_recovery_codes::is_used.eq(true),
                            mfa_recovery_codes::used_at.eq(now),
                        ))
                        .execute(&conn)?;
                    
                    Ok(true)
                } else {
                    Ok(false)
                }
            })
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e: diesel::result::Error| AuthError::DatabaseError(format!("Transaction error: {}", e)))?;
        
        Ok(result)
    }

    pub async fn delete_recovery_codes(&self, user_id: Uuid) -> Result<(), AuthError> {
        let conn = self.get_conn()?;
        
        tokio::task::spawn_blocking(move || {
            diesel::delete(mfa_recovery_codes::table.filter(mfa_recovery_codes::user_id.eq(user_id)))
                .execute(&conn)
        })
        .await
        .map_err(|e| AuthError::DatabaseError(format!("Task join error: {}", e)))?
        .map_err(|e| AuthError::DatabaseError(format!("Delete error: {}", e)))?;
        
        Ok(())
    }
}
