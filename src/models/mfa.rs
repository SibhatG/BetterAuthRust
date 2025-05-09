use crate::schema::mfa_recovery_codes;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = mfa_recovery_codes)]
pub struct MfaRecoveryCode {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub is_used: bool,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = mfa_recovery_codes)]
pub struct NewMfaRecoveryCode {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct MfaRecoveryCodesResponse {
    pub recovery_codes: Vec<String>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct MfaLoginRequest {
    pub username_or_email: String,
    pub password: String,
    pub mfa_code: Option<String>,
    pub recovery_code: Option<String>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct MfaVerifyRequest {
    pub mfa_code: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct MfaRecoveryRequest {
    pub recovery_code: String,
}

#[derive(Debug, Serialize)]
pub struct MfaVerifyResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub user: super::user::UserResponse,
}
