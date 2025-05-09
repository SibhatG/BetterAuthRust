/**
 * Type definitions for authentication models
 */
export interface User {
    id: string;
    username: string;
    email: string;
    is_email_verified: boolean;
    mfa_enabled: boolean;
}
export interface RegisterRequest {
    username: string;
    email: string;
    password: string;
    password_confirmation: string;
}
export interface RegisterResponse {
    user: User;
    message: string;
}
export interface LoginRequest {
    username_or_email: string;
    password: string;
}
export interface LoginResponse {
    access_token: string;
    refresh_token: string;
    token_type: string;
    expires_in: number;
    user: User;
}
export interface ErrorResponse {
    status: string;
    code: string;
    message: string;
}
export interface WebAuthnLoginStartRequest {
    username_or_email: string;
}
export interface WebAuthnCredential {
    credential_id: string;
    public_key: string;
    counter: number;
    created_at: string;
    last_used_at: string | null;
}
export interface WebAuthnOptions {
    challenge: string;
    rp_id: string;
    rp_name: string;
    user_id: string;
    username: string;
    timeout: number;
}
export interface WebAuthnRegisterStartResponse {
    registration_id: string;
    options: WebAuthnOptions;
}
export interface WebAuthnAuthenticatorResponse {
    client_data_json: string;
    attestation_object?: string;
    authenticator_data?: string;
    signature?: string;
    user_handle?: string;
}
export interface WebAuthnCredentialResponse {
    id: string;
    raw_id: string;
    response: WebAuthnAuthenticatorResponse;
    type: string;
}
export interface WebAuthnRegisterCompleteRequest {
    registration_id: string;
    credential: WebAuthnCredentialResponse;
}
export interface WebAuthnAuthenticateStartResponse {
    authentication_id: string;
    options: WebAuthnOptions;
}
export interface WebAuthnAuthenticateCompleteRequest {
    authentication_id: string;
    credential: WebAuthnCredentialResponse;
}
