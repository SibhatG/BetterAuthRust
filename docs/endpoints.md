# Better Auth API Endpoints

This document provides a comprehensive reference for all API endpoints in the Better Auth system.

## Table of Contents

1. [Authentication](#authentication)
2. [WebAuthn](#webauthn)
3. [Risk Scoring](#risk-scoring)
4. [Breach Detection](#breach-detection)
5. [Proxy Email](#proxy-email)
6. [Hybrid Encryption](#hybrid-encryption)
7. [Accessibility](#accessibility)
8. [HIPAA Compliance](#hipaa-compliance)

## Authentication

### Health Check

```
GET /health
```

Response:
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

### Register

```
POST /api/auth/register
```

Request:
```json
{
  "username": "testuser",
  "email": "test@example.com",
  "password": "securePass123",
  "password_confirmation": "securePass123"
}
```

Response:
```json
{
  "user": {
    "id": "f9ba34a8-9a55-44e0-8686-f7d95494fc2c",
    "username": "testuser",
    "email": "test@example.com",
    "is_email_verified": false,
    "mfa_enabled": false
  },
  "message": "User registered successfully. Please check your email to verify your account."
}
```

### Login

```
POST /api/auth/login
```

Request:
```json
{
  "username_or_email": "testuser",
  "password": "securePass123"
}
```

Response:
```json
{
  "access_token": "63962e4f-e57e-422d-80f7-3444519a3338",
  "refresh_token": "3db2c365-f87f-40f2-8d7f-5b3d56dcce89",
  "token_type": "Bearer",
  "expires_in": 3600,
  "user": {
    "id": "f9ba34a8-9a55-44e0-8686-f7d95494fc2c",
    "username": "testuser",
    "email": "test@example.com",
    "is_email_verified": false,
    "mfa_enabled": false
  }
}
```

### Get Current User

```
GET /api/auth/user
```

Headers:
```
Authorization: Bearer {access_token}
```

Response:
```json
{
  "id": "f9ba34a8-9a55-44e0-8686-f7d95494fc2c",
  "username": "testuser",
  "email": "test@example.com",
  "is_email_verified": false,
  "mfa_enabled": false
}
```

### Logout

```
POST /api/auth/logout
```

Headers:
```
Authorization: Bearer {access_token}
```

Response:
```json
{
  "message": "Successfully logged out"
}
```

## WebAuthn

### Start WebAuthn Registration

```
POST /api/auth/webauthn/register/start
```

Headers:
```
Authorization: Bearer {access_token}
```

Response:
```json
{
  "registration_id": "550e8400-e29b-41d4-a716-446655440000",
  "options": {
    "challenge": "base64-encoded-challenge",
    "rp_id": "better-auth.example.com",
    "rp_name": "Better Auth Example",
    "user_id": "f9ba34a8-9a55-44e0-8686-f7d95494fc2c",
    "username": "testuser",
    "timeout": 60000
  }
}
```

### Complete WebAuthn Registration

```
POST /api/auth/webauthn/register/complete
```

Headers:
```
Authorization: Bearer {access_token}
```

Request:
```json
{
  "registration_id": "550e8400-e29b-41d4-a716-446655440000",
  "credential": {
    "id": "credential-id",
    "raw_id": "base64-encoded-raw-id",
    "response": {
      "client_data_json": "base64-encoded-client-data",
      "attestation_object": "base64-encoded-attestation"
    },
    "type": "public-key"
  }
}
```

Response:
```json
{
  "credential_id": "credential-id",
  "public_key": "base64-encoded-public-key",
  "counter": 0,
  "created_at": "2025-05-09T18:00:00Z",
  "last_used_at": null
}
```

### Start WebAuthn Login

```
POST /api/auth/webauthn/login/start
```

Request:
```json
{
  "username_or_email": "testuser"
}
```

Response:
```json
{
  "authentication_id": "550e8400-e29b-41d4-a716-446655440000",
  "options": {
    "challenge": "base64-encoded-challenge",
    "rp_id": "better-auth.example.com",
    "rp_name": "Better Auth Example",
    "user_id": "f9ba34a8-9a55-44e0-8686-f7d95494fc2c",
    "username": "authentication",
    "timeout": 60000
  }
}
```

### Complete WebAuthn Login

```
POST /api/auth/webauthn/login/complete
```

Request:
```json
{
  "authentication_id": "550e8400-e29b-41d4-a716-446655440000",
  "credential": {
    "id": "credential-id",
    "raw_id": "base64-encoded-raw-id",
    "response": {
      "client_data_json": "base64-encoded-client-data",
      "authenticator_data": "base64-encoded-authenticator-data",
      "signature": "base64-encoded-signature",
      "user_handle": "base64-encoded-user-handle"
    },
    "type": "public-key"
  }
}
```

Response:
```json
{
  "access_token": "63962e4f-e57e-422d-80f7-3444519a3338",
  "refresh_token": "3db2c365-f87f-40f2-8d7f-5b3d56dcce89",
  "token_type": "Bearer",
  "expires_in": 3600,
  "user": {
    "id": "f9ba34a8-9a55-44e0-8686-f7d95494fc2c",
    "username": "testuser",
    "email": "test@example.com",
    "is_email_verified": false,
    "mfa_enabled": false
  }
}
```

## Risk Scoring

### Get Risk Analysis

```
GET /api/risk/analysis
```

Headers:
```
Authorization: Bearer {access_token}
```

Response:
```json
{
  "score": 35,
  "factors": [
    {
      "name": "new_device",
      "description": "Login from a new device",
      "weight": 20
    },
    {
      "name": "new_location",
      "description": "Login from a new location",
      "weight": 15
    }
  ],
  "action": "RequireMfa"
}
```

## Breach Detection

### Check Breach Status

```
GET /api/breach/check
```

Headers:
```
Authorization: Bearer {access_token}
```

Response:
```json
{
  "is_breached": true,
  "breaches": [
    {
      "breach_date": "2024-01-15T00:00:00Z",
      "source": "ExampleSite.com",
      "data_types": ["email", "username", "IP address"],
      "description": "ExampleSite suffered a data breach exposing user emails and usernames"
    }
  ],
  "password_compromised": false,
  "action_required": "None"
}
```

## Proxy Email

### Create Proxy Email

```
POST /api/email/create
```

Headers:
```
Authorization: Bearer {access_token}
```

Request:
```json
{
  "label": "Shopping"
}
```

Response:
```json
{
  "proxy_address": "abcdef123456@proxy.betterauth.com",
  "real_address": "test@example.com",
  "created_at": "2025-05-09T18:00:00Z",
  "label": "Shopping",
  "status": "Active",
  "forwarding_enabled": true
}
```

### List Proxy Emails

```
GET /api/email/list
```

Headers:
```
Authorization: Bearer {access_token}
```

Response:
```json
{
  "proxy_emails": [
    {
      "proxy_address": "abcdef123456@proxy.betterauth.com",
      "real_address": "test@example.com",
      "created_at": "2025-05-09T18:00:00Z",
      "label": "Shopping",
      "status": "Active",
      "forwarding_enabled": true
    }
  ]
}
```

### Update Proxy Email Status

```
PATCH /api/email/status
```

Headers:
```
Authorization: Bearer {access_token}
```

Request:
```json
{
  "proxy_address": "abcdef123456@proxy.betterauth.com",
  "status": "Disabled"
}
```

Response:
```json
{
  "proxy_address": "abcdef123456@proxy.betterauth.com",
  "real_address": "test@example.com",
  "created_at": "2025-05-09T18:00:00Z",
  "label": "Shopping",
  "status": "Disabled",
  "forwarding_enabled": true
}
```

### Delete Proxy Email

```
DELETE /api/email/delete
```

Headers:
```
Authorization: Bearer {access_token}
```

Request:
```json
{
  "proxy_address": "abcdef123456@proxy.betterauth.com"
}
```

Response:
```json
{
  "success": true
}
```

### Get Forwarding Preferences

```
GET /api/email/preferences
```

Headers:
```
Authorization: Bearer {access_token}
```

Response:
```json
{
  "forward_all": true,
  "spam_filter_level": "Medium",
  "blocked_senders": [
    "spam@example.com"
  ],
  "allowed_senders": [
    "important@example.com"
  ]
}
```

### Update Forwarding Preferences

```
PATCH /api/email/preferences
```

Headers:
```
Authorization: Bearer {access_token}
```

Request:
```json
{
  "forward_all": true,
  "spam_filter_level": "High",
  "blocked_senders": [
    "spam@example.com",
    "unwanted@example.com"
  ],
  "allowed_senders": [
    "important@example.com"
  ]
}
```

Response:
```json
{
  "forward_all": true,
  "spam_filter_level": "High",
  "blocked_senders": [
    "spam@example.com",
    "unwanted@example.com"
  ],
  "allowed_senders": [
    "important@example.com"
  ]
}
```

## Hybrid Encryption

### Get Public Keys

```
GET /api/encryption/keys
```

Headers:
```
Authorization: Bearer {access_token}
```

Response:
```json
{
  "user_id": "f9ba34a8-9a55-44e0-8686-f7d95494fc2c",
  "rsa_public_key": "RSA_PUB_550e8400-e29b-41d4-a716-446655440000",
  "kyber_public_key": "KYBER_PUB_550e8400-e29b-41d4-a716-446655440000",
  "created_at": "2025-05-09T18:00:00Z"
}
```

### Encrypt Data

```
POST /api/encryption/encrypt
```

Headers:
```
Authorization: Bearer {access_token}
```

Request:
```json
{
  "data": "This is sensitive data that needs encryption"
}
```

Response:
```json
{
  "encrypted_data": {
    "rsa_encrypted_key": "RSA_ENC(SYM_KEY_550e8400-e29b-41d4-a716-446655440000)",
    "kyber_encrypted_key": "KYBER_ENC(SYM_KEY_550e8400-e29b-41d4-a716-446655440000)",
    "encrypted_data": "base64-encoded-encrypted-data",
    "nonce": "550e8400-e29b-41d4-a716-446655440000",
    "algorithm": "AES-256-GCM"
  }
}
```

### Decrypt Data

```
POST /api/encryption/decrypt
```

Headers:
```
Authorization: Bearer {access_token}
```

Request:
```json
{
  "encrypted_data": {
    "rsa_encrypted_key": "RSA_ENC(SYM_KEY_550e8400-e29b-41d4-a716-446655440000)",
    "kyber_encrypted_key": "KYBER_ENC(SYM_KEY_550e8400-e29b-41d4-a716-446655440000)",
    "encrypted_data": "base64-encoded-encrypted-data",
    "nonce": "550e8400-e29b-41d4-a716-446655440000",
    "algorithm": "AES-256-GCM"
  }
}
```

Response:
```json
{
  "data": "This is sensitive data that needs encryption"
}
```

### Rotate Keys

```
POST /api/encryption/rotate
```

Headers:
```
Authorization: Bearer {access_token}
```

Response:
```json
{
  "key_pair": {
    "user_id": "f9ba34a8-9a55-44e0-8686-f7d95494fc2c",
    "rsa_public_key": "RSA_PUB_550e8400-e29b-41d4-a716-446655440000",
    "kyber_public_key": "KYBER_PUB_550e8400-e29b-41d4-a716-446655440000",
    "created_at": "2025-05-09T18:00:00Z"
  }
}
```

## Accessibility

### Get Accessibility Preferences

```
GET /api/accessibility/preferences
```

Headers:
```
Authorization: Bearer {access_token}
```

Response:
```json
{
  "high_contrast": false,
  "large_text": true,
  "screen_reader_optimized": false,
  "reduced_motion": true,
  "voice_commands_enabled": false,
  "keyboard_navigation": true,
  "additional_settings": {
    "color_blind_mode": "deuteranopia"
  }
}
```

### Update Accessibility Preferences

```
PATCH /api/accessibility/preferences
```

Headers:
```
Authorization: Bearer {access_token}
```

Request:
```json
{
  "high_contrast": true,
  "large_text": true,
  "screen_reader_optimized": true,
  "reduced_motion": true,
  "voice_commands_enabled": false,
  "keyboard_navigation": true
}
```

Response:
```json
{
  "high_contrast": true,
  "large_text": true,
  "screen_reader_optimized": true,
  "reduced_motion": true,
  "voice_commands_enabled": false,
  "keyboard_navigation": true,
  "additional_settings": {
    "color_blind_mode": "deuteranopia"
  }
}
```

### Get CSS Variables

```
GET /api/accessibility/css
```

Headers:
```
Authorization: Bearer {access_token}
```

Response:
```
--background-color: #000000;
--text-color: #ffffff;
--link-color: #ffff00;
--input-bg-color: #333333;
--input-text-color: #ffffff;
--button-bg-color: #0066ff;
--button-text-color: #ffffff;
--contrast-ratio: 7;
--font-size-base: 18px;
--font-size-large: 24px;
--font-size-small: 16px;
--line-height: 1.5;
--transition-speed: 0s;
--animation-speed: 0s;
--focus-outline-width: 3px;
--focus-outline-style: solid;
--focus-outline-color: #ff6600;
```

### Get CAPTCHA Alternative

```
GET /api/accessibility/captcha
```

Headers:
```
Authorization: Bearer {access_token}
```

Response:
```json
{
  "captcha_type": "Audio"
}
```

### Process Voice Command

```
POST /api/accessibility/voice
```

Headers:
```
Authorization: Bearer {access_token}
```

Request: Binary audio data

Response:
```json
{
  "command": "login",
  "confidence": 0.95,
  "action": "submitLoginForm"
}
```

## HIPAA Compliance

### Get User Role

```
GET /api/hipaa/role
```

Headers:
```
Authorization: Bearer {access_token}
```

Response:
```json
{
  "role": "Doctor"
}
```

### Get Role Permissions

```
GET /api/hipaa/permissions
```

Headers:
```
Authorization: Bearer {access_token}
```

Response:
```json
{
  "permissions": [
    {
      "resource_type": "PatientRecord",
      "allowed_access_types": ["View", "Create", "Update"]
    },
    {
      "resource_type": "MedicalImages",
      "allowed_access_types": ["View"]
    }
  ]
}
```

### Check Permission

```
POST /api/hipaa/permission/check
```

Headers:
```
Authorization: Bearer {access_token}
```

Request:
```json
{
  "resource_type": "PatientRecord",
  "access_type": "Update"
}
```

Response:
```json
{
  "has_permission": true
}
```

### Get Active Sessions

```
GET /api/hipaa/sessions
```

Headers:
```
Authorization: Bearer {access_token}
```

Response:
```json
{
  "sessions": [
    {
      "session_id": "550e8400-e29b-41d4-a716-446655440000",
      "user_id": "f9ba34a8-9a55-44e0-8686-f7d95494fc2c",
      "created_at": "2025-05-09T17:00:00Z",
      "last_activity": "2025-05-09T18:00:00Z",
      "ip_address": "192.168.1.1",
      "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
      "role": "Doctor"
    }
  ]
}
```

### Terminate Session

```
POST /api/hipaa/sessions/terminate
```

Headers:
```
Authorization: Bearer {access_token}
```

Request:
```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

Response:
```json
{
  "success": true
}
```

### Get Access Logs

```
GET /api/hipaa/logs?start_date=2025-05-01T00:00:00Z&end_date=2025-05-09T23:59:59Z
```

Headers:
```
Authorization: Bearer {access_token}
```

Response:
```json
{
  "logs": [
    {
      "log_id": "550e8400-e29b-41d4-a716-446655440000",
      "user_id": "f9ba34a8-9a55-44e0-8686-f7d95494fc2c",
      "user_name": "Dr. Jane Smith",
      "user_role": "Doctor",
      "resource_id": "patient-123",
      "resource_type": "PatientRecord",
      "access_type": "View",
      "timestamp": "2025-05-09T15:30:00Z",
      "ip_address": "192.168.1.1",
      "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
      "reason": "Routine checkup"
    }
  ]
}
```

### Log Emergency Access

```
POST /api/hipaa/emergency
```

Headers:
```
Authorization: Bearer {access_token}
```

Request:
```json
{
  "reason": "Patient emergency - critical care needed",
  "resources": ["patient-123", "lab-results-456"]
}
```

Response:
```json
{
  "access_id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "f9ba34a8-9a55-44e0-8686-f7d95494fc2c",
  "timestamp": "2025-05-09T18:00:00Z",
  "reason": "Patient emergency - critical care needed",
  "resources_accessed": ["patient-123", "lab-results-456"]
}
```

### Review Emergency Access

```
POST /api/hipaa/emergency/review
```

Headers:
```
Authorization: Bearer {access_token}
```

Request:
```json
{
  "access_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

Response:
```json
{
  "success": true
}
```

### Generate Audit Report

```
POST /api/hipaa/report
```

Headers:
```
Authorization: Bearer {access_token}
```

Request:
```json
{
  "start_date": "2025-05-01T00:00:00Z",
  "end_date": "2025-05-09T23:59:59Z"
}
```

Response:
```json
{
  "report_url": "https://better-auth.example.com/reports/audit-550e8400-e29b-41d4-a716-446655440000.pdf"
}
```