-- Create WebAuthn credentials table for passwordless authentication
CREATE TABLE IF NOT EXISTS webauthn_credentials (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    credential_id TEXT NOT NULL,
    public_key TEXT NOT NULL,
    counter INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    last_used_at TIMESTAMP WITH TIME ZONE,
    device_name TEXT,
    
    CONSTRAINT uc_credential_id UNIQUE (credential_id)
);

-- Create indexes
CREATE INDEX idx_webauthn_credentials_user_id ON webauthn_credentials(user_id);