-- Drop WebAuthn credentials table and related objects
DROP INDEX IF EXISTS idx_webauthn_credentials_user_id;
DROP TABLE IF EXISTS webauthn_credentials;