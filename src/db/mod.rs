pub mod memory;
pub mod postgres;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use r2d2::Error as R2D2Error;
use std::sync::Arc;

use crate::config::Config;
use crate::errors::AuthError;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub enum Database {
    Postgres(postgres::PostgresDb),
    Memory(memory::MemoryDb),
}

pub struct DatabaseConnection {
    db: Database,
}

impl DatabaseConnection {
    pub fn new_postgres(pool: PgPool) -> Self {
        Self {
            db: Database::Postgres(postgres::PostgresDb::new(pool)),
        }
    }

    pub fn new_memory() -> Self {
        Self {
            db: Database::Memory(memory::MemoryDb::new()),
        }
    }

    // User methods
    pub async fn create_user(&self, user: crate::models::NewUser) -> Result<crate::models::User, AuthError> {
        match &self.db {
            Database::Postgres(db) => db.create_user(user).await,
            Database::Memory(db) => db.create_user(user).await,
        }
    }

    pub async fn find_user_by_id(&self, id: uuid::Uuid) -> Result<crate::models::User, AuthError> {
        match &self.db {
            Database::Postgres(db) => db.find_user_by_id(id).await,
            Database::Memory(db) => db.find_user_by_id(id).await,
        }
    }

    pub async fn find_user_by_username(&self, username: &str) -> Result<crate::models::User, AuthError> {
        match &self.db {
            Database::Postgres(db) => db.find_user_by_username(username).await,
            Database::Memory(db) => db.find_user_by_username(username).await,
        }
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<crate::models::User, AuthError> {
        match &self.db {
            Database::Postgres(db) => db.find_user_by_email(email).await,
            Database::Memory(db) => db.find_user_by_email(email).await,
        }
    }

    pub async fn find_user_by_username_or_email(&self, username_or_email: &str) -> Result<crate::models::User, AuthError> {
        match &self.db {
            Database::Postgres(db) => db.find_user_by_username_or_email(username_or_email).await,
            Database::Memory(db) => db.find_user_by_username_or_email(username_or_email).await,
        }
    }

    pub async fn find_user_by_verification_token(&self, token: &str) -> Result<crate::models::User, AuthError> {
        match &self.db {
            Database::Postgres(db) => db.find_user_by_verification_token(token).await,
            Database::Memory(db) => db.find_user_by_verification_token(token).await,
        }
    }

    pub async fn find_user_by_reset_token(&self, token: &str) -> Result<crate::models::User, AuthError> {
        match &self.db {
            Database::Postgres(db) => db.find_user_by_reset_token(token).await,
            Database::Memory(db) => db.find_user_by_reset_token(token).await,
        }
    }

    pub async fn user_exists_by_username(&self, username: &str) -> Result<bool, AuthError> {
        match &self.db {
            Database::Postgres(db) => db.user_exists_by_username(username).await,
            Database::Memory(db) => db.user_exists_by_username(username).await,
        }
    }

    pub async fn user_exists_by_email(&self, email: &str) -> Result<bool, AuthError> {
        match &self.db {
            Database::Postgres(db) => db.user_exists_by_email(email).await,
            Database::Memory(db) => db.user_exists_by_email(email).await,
        }
    }

    pub async fn update_last_login(&self, id: uuid::Uuid) -> Result<(), AuthError> {
        match &self.db {
            Database::Postgres(db) => db.update_last_login(id).await,
            Database::Memory(db) => db.update_last_login(id).await,
        }
    }

    pub async fn update_verification_token(&self, id: uuid::Uuid, token: &str) -> Result<(), AuthError> {
        match &self.db {
            Database::Postgres(db) => db.update_verification_token(id, token).await,
            Database::Memory(db) => db.update_verification_token(id, token).await,
        }
    }

    pub async fn update_password_reset_token(&self, id: uuid::Uuid, token: &str) -> Result<(), AuthError> {
        match &self.db {
            Database::Postgres(db) => db.update_password_reset_token(id, token).await,
            Database::Memory(db) => db.update_password_reset_token(id, token).await,
        }
    }

    pub async fn update_password(&self, id: uuid::Uuid, password_hash: &str) -> Result<(), AuthError> {
        match &self.db {
            Database::Postgres(db) => db.update_password(id, password_hash).await,
            Database::Memory(db) => db.update_password(id, password_hash).await,
        }
    }

    pub async fn verify_email(&self, id: uuid::Uuid) -> Result<crate::models::User, AuthError> {
        match &self.db {
            Database::Postgres(db) => db.verify_email(id).await,
            Database::Memory(db) => db.verify_email(id).await,
        }
    }

    pub async fn update_mfa_secret(&self, id: uuid::Uuid, secret: &str) -> Result<(), AuthError> {
        match &self.db {
            Database::Postgres(db) => db.update_mfa_secret(id, secret).await,
            Database::Memory(db) => db.update_mfa_secret(id, secret).await,
        }
    }

    pub async fn enable_mfa(&self, id: uuid::Uuid) -> Result<(), AuthError> {
        match &self.db {
            Database::Postgres(db) => db.enable_mfa(id).await,
            Database::Memory(db) => db.enable_mfa(id).await,
        }
    }

    pub async fn disable_mfa(&self, id: uuid::Uuid) -> Result<crate::models::User, AuthError> {
        match &self.db {
            Database::Postgres(db) => db.disable_mfa(id).await,
            Database::Memory(db) => db.disable_mfa(id).await,
        }
    }

    // Session methods
    pub async fn create_session(&self, session: crate::models::NewSession) -> Result<crate::models::Session, AuthError> {
        match &self.db {
            Database::Postgres(db) => db.create_session(session).await,
            Database::Memory(db) => db.create_session(session).await,
        }
    }

    pub async fn find_session_by_id(&self, id: uuid::Uuid) -> Result<crate::models::Session, AuthError> {
        match &self.db {
            Database::Postgres(db) => db.find_session_by_id(id).await,
            Database::Memory(db) => db.find_session_by_id(id).await,
        }
    }

    pub async fn find_session_by_token(&self, token: &str) -> Result<crate::models::Session, AuthError> {
        match &self.db {
            Database::Postgres(db) => db.find_session_by_token(token).await,
            Database::Memory(db) => db.find_session_by_token(token).await,
        }
    }

    pub async fn find_sessions_by_user_id(&self, user_id: uuid::Uuid) -> Result<Vec<crate::models::Session>, AuthError> {
        match &self.db {
            Database::Postgres(db) => db.find_sessions_by_user_id(user_id).await,
            Database::Memory(db) => db.find_sessions_by_user_id(user_id).await,
        }
    }

    pub async fn revoke_session(&self, id: uuid::Uuid) -> Result<(), AuthError> {
        match &self.db {
            Database::Postgres(db) => db.revoke_session(id).await,
            Database::Memory(db) => db.revoke_session(id).await,
        }
    }

    pub async fn revoke_all_sessions(&self, user_id: uuid::Uuid) -> Result<(), AuthError> {
        match &self.db {
            Database::Postgres(db) => db.revoke_all_sessions(user_id).await,
            Database::Memory(db) => db.revoke_all_sessions(user_id).await,
        }
    }

    // MFA Recovery codes methods
    pub async fn create_recovery_code(&self, code: crate::models::NewMfaRecoveryCode) -> Result<crate::models::MfaRecoveryCode, AuthError> {
        match &self.db {
            Database::Postgres(db) => db.create_recovery_code(code).await,
            Database::Memory(db) => db.create_recovery_code(code).await,
        }
    }

    pub async fn use_recovery_code(&self, user_id: uuid::Uuid, code: &str) -> Result<bool, AuthError> {
        match &self.db {
            Database::Postgres(db) => db.use_recovery_code(user_id, code).await,
            Database::Memory(db) => db.use_recovery_code(user_id, code).await,
        }
    }

    pub async fn delete_recovery_codes(&self, user_id: uuid::Uuid) -> Result<(), AuthError> {
        match &self.db {
            Database::Postgres(db) => db.delete_recovery_codes(user_id).await,
            Database::Memory(db) => db.delete_recovery_codes(user_id).await,
        }
    }
}

pub fn init_db(config: &Config) -> Result<Arc<DatabaseConnection>, AuthError> {
    // Get database connection from environment
    let database_url = &config.database.url;
    
    // Create database connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(config.database.pool_size)
        .build(manager)
        .map_err(|e: R2D2Error| {
            AuthError::DatabaseError(format!("Failed to create connection pool: {}", e))
        })?;
    
    // Create database connection
    let db = DatabaseConnection::new_postgres(pool);
    
    Ok(Arc::new(db))
}
