// @generated automatically by Diesel CLI.

diesel::table! {
    mfa_recovery_codes (id) {
        id -> Uuid,
        user_id -> Uuid,
        code -> Text,
        is_used -> Bool,
        used_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    sessions (id) {
        id -> Uuid,
        user_id -> Uuid,
        refresh_token -> Text,
        user_agent -> Nullable<Text>,
        ip_address -> Nullable<Text>,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        is_revoked -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        is_email_verified -> Bool,
        email_verification_token -> Nullable<Text>,
        email_verification_sent_at -> Nullable<Timestamptz>,
        password_reset_token -> Nullable<Text>,
        password_reset_sent_at -> Nullable<Timestamptz>,
        mfa_enabled -> Bool,
        mfa_secret -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        last_login_at -> Nullable<Timestamptz>,
        is_active -> Bool,
        is_admin -> Bool,
    }
}

diesel::joinable!(mfa_recovery_codes -> users (user_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    mfa_recovery_codes,
    sessions,
    users,
);
