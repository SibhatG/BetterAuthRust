# Better Auth API Documentation

## Authentication Endpoints

### Register a New User

`POST /api/auth/register`

Creates a new user account.

**Request Body:**

```json
{
  "username": "johndoe",
  "email": "john@example.com",
  "password": "securePassword123",
  "password_confirmation": "securePassword123"
}
```

**Success Response: 201 Created**

```json
{
  "user": {
    "id": "e29a9d8a-7c2f-4bc1-a533-b7d60979bf54",
    "username": "johndoe",
    "email": "john@example.com",
    "is_email_verified": false,
    "mfa_enabled": false
  },
  "message": "User registered successfully. Please check your email to verify your account."
}
```

**Error Responses:**

- `400 Bad Request`: Validation error, username exists, or email exists
- `500 Internal Server Error`: Server error

### Login

`POST /api/auth/login`

Authenticates a user and returns access tokens.

**Request Body:**

```json
{
  "username_or_email": "johndoe",
  "password": "securePassword123"
}
```

**Success Response: 200 OK**

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "a722c63c-d1d1-4859-8fc9-27b2b8f69d0a",
  "token_type": "Bearer",
  "expires_in": 3600,
  "user": {
    "id": "e29a9d8a-7c2f-4bc1-a533-b7d60979bf54",
    "username": "johndoe",
    "email": "john@example.com",
    "is_email_verified": false,
    "mfa_enabled": false
  }
}
```

**Error Responses:**

- `401 Unauthorized`: Invalid credentials
- `500 Internal Server Error`: Server error

### Get Current User

`GET /api/users/me`

Returns the currently authenticated user's details.

**Request Headers:**

```
Authorization: Bearer <access_token>
```

**Success Response: 200 OK**

```json
{
  "id": "e29a9d8a-7c2f-4bc1-a533-b7d60979bf54",
  "username": "johndoe",
  "email": "john@example.com",
  "is_email_verified": false,
  "mfa_enabled": false
}
```

**Error Responses:**

- `401 Unauthorized`: Authentication required
- `500 Internal Server Error`: Server error

## WebAuthn Endpoints

### Start WebAuthn Registration

`POST /api/auth/webauthn/register/start`

Starts the registration process for a new WebAuthn credential.

**Request Headers:**

```
Authorization: Bearer <access_token>
```

**Success Response: 200 OK**

```json
{
  "challenge": "randomChallenge",
  "rp": {
    "id": "better-auth.example.com",
    "name": "Better Auth"
  },
  "user": {
    "id": "base64UserId",
    "name": "johndoe",
    "displayName": "John Doe"
  },
  "pubKeyCredParams": [
    { "type": "public-key", "alg": -7 },
    { "type": "public-key", "alg": -257 }
  ],
  "timeout": 60000,
  "attestation": "direct",
  "excludeCredentials": []
}
```

**Error Responses:**

- `401 Unauthorized`: Authentication required
- `500 Internal Server Error`: Server error

### Complete WebAuthn Registration

`POST /api/auth/webauthn/register/complete`

Completes the registration of a new WebAuthn credential.

**Request Headers:**

```
Authorization: Bearer <access_token>
```

**Request Body:**

```json
{
  "id": "credentialId",
  "rawId": "base64RawId",
  "response": {
    "attestationObject": "base64AttestationObject",
    "clientDataJSON": "base64ClientDataJSON"
  },
  "type": "public-key"
}
```

**Success Response: 200 OK**

```json
{
  "status": "success",
  "message": "WebAuthn credential registered successfully"
}
```

**Error Responses:**

- `401 Unauthorized`: Authentication required
- `400 Bad Request`: Invalid registration response
- `500 Internal Server Error`: Server error

### Start WebAuthn Login

`POST /api/auth/webauthn/login/start`

Starts the WebAuthn authentication process.

**Request Body:**

```json
{
  "username_or_email": "johndoe"
}
```

**Success Response: 200 OK**

```json
{
  "challenge": "randomChallenge",
  "timeout": 60000,
  "rpId": "better-auth.example.com",
  "allowCredentials": [
    {
      "id": "base64CredentialId",
      "type": "public-key"
    }
  ],
  "userVerification": "preferred"
}
```

**Error Responses:**

- `400 Bad Request`: User not found or has no WebAuthn credentials
- `500 Internal Server Error`: Server error

### Complete WebAuthn Login

`POST /api/auth/webauthn/login/complete`

Completes the WebAuthn authentication process.

**Request Body:**

```json
{
  "id": "credentialId",
  "rawId": "base64RawId",
  "response": {
    "authenticatorData": "base64AuthenticatorData",
    "clientDataJSON": "base64ClientDataJSON",
    "signature": "base64Signature",
    "userHandle": "base64UserHandle"
  },
  "type": "public-key"
}
```

**Success Response: 200 OK**

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "a722c63c-d1d1-4859-8fc9-27b2b8f69d0a",
  "token_type": "Bearer",
  "expires_in": 3600,
  "user": {
    "id": "e29a9d8a-7c2f-4bc1-a533-b7d60979bf54",
    "username": "johndoe",
    "email": "john@example.com",
    "is_email_verified": false,
    "mfa_enabled": false
  }
}
```

**Error Responses:**

- `400 Bad Request`: Invalid authentication response
- `401 Unauthorized`: Authentication failed
- `500 Internal Server Error`: Server error

## Proxy Email Endpoints

### Create Proxy Email

`POST /api/email/create`

Creates a new proxy email address.

**Request Headers:**

```
Authorization: Bearer <access_token>
```

**Request Body:**

```json
{
  "label": "Shopping Site"
}
```

**Success Response: 201 Created**

```json
{
  "proxy_address": "random123@proxy.betterauth.com",
  "label": "Shopping Site",
  "created_at": "2025-05-09T12:34:56Z",
  "status": "Active"
}
```

**Error Responses:**

- `401 Unauthorized`: Authentication required
- `500 Internal Server Error`: Server error

### List Proxy Emails

`GET /api/email/list`

Lists all proxy email addresses for the current user.

**Request Headers:**

```
Authorization: Bearer <access_token>
```

**Success Response: 200 OK**

```json
{
  "proxy_emails": [
    {
      "proxy_address": "random123@proxy.betterauth.com",
      "label": "Shopping Site",
      "created_at": "2025-05-09T12:34:56Z",
      "status": "Active",
      "forwarding_enabled": true
    },
    {
      "proxy_address": "random456@proxy.betterauth.com",
      "label": "Social Media",
      "created_at": "2025-05-08T10:22:33Z",
      "status": "Active",
      "forwarding_enabled": true
    }
  ]
}
```

**Error Responses:**

- `401 Unauthorized`: Authentication required
- `500 Internal Server Error`: Server error

### Update Proxy Email Status

`PATCH /api/email/status`

Updates the status of a proxy email address.

**Request Headers:**

```
Authorization: Bearer <access_token>
```

**Request Body:**

```json
{
  "proxy_address": "random123@proxy.betterauth.com",
  "status": "Disabled",
  "forwarding_enabled": false
}
```

**Success Response: 200 OK**

```json
{
  "proxy_address": "random123@proxy.betterauth.com",
  "status": "Disabled",
  "forwarding_enabled": false
}
```

**Error Responses:**

- `401 Unauthorized`: Authentication required
- `404 Not Found`: Proxy email not found
- `500 Internal Server Error`: Server error

## Risk Scoring Endpoints

### Get Risk Analysis

`GET /api/risk/analysis`

Returns the risk analysis for the current user's recent logins.

**Request Headers:**

```
Authorization: Bearer <access_token>
```

**Success Response: 200 OK**

```json
{
  "risk_score": 25,
  "risk_factors": [
    {
      "name": "new_device",
      "description": "Login from a new device",
      "weight": 20
    },
    {
      "name": "unusual_time",
      "description": "Login at an unusual time for this user",
      "weight": 15
    }
  ],
  "recommended_action": "RequireMfa"
}
```

**Error Responses:**

- `401 Unauthorized`: Authentication required
- `500 Internal Server Error`: Server error

## Breach Detection Endpoints

### Check Breach Status

`GET /api/security/breach-check`

Checks if the user's credentials have been compromised in known breaches.

**Request Headers:**

```
Authorization: Bearer <access_token>
```

**Success Response: 200 OK**

```json
{
  "is_breached": false,
  "breaches": [],
  "password_compromised": false,
  "action_required": "None"
}
```

**Error Responses:**

- `401 Unauthorized`: Authentication required
- `500 Internal Server Error`: Server error

## Accessibility Endpoints

### Get Accessibility Preferences

`GET /api/accessibility/preferences`

Returns the user's accessibility preferences.

**Request Headers:**

```
Authorization: Bearer <access_token>
```

**Success Response: 200 OK**

```json
{
  "high_contrast": false,
  "large_text": true,
  "screen_reader_optimized": false,
  "reduced_motion": true,
  "voice_commands_enabled": false,
  "keyboard_navigation": true
}
```

**Error Responses:**

- `401 Unauthorized`: Authentication required
- `500 Internal Server Error`: Server error

### Update Accessibility Preferences

`PATCH /api/accessibility/preferences`

Updates the user's accessibility preferences.

**Request Headers:**

```
Authorization: Bearer <access_token>
```

**Request Body:**

```json
{
  "high_contrast": true,
  "large_text": true
}
```

**Success Response: 200 OK**

```json
{
  "high_contrast": true,
  "large_text": true,
  "screen_reader_optimized": false,
  "reduced_motion": true,
  "voice_commands_enabled": false,
  "keyboard_navigation": true
}
```

**Error Responses:**

- `401 Unauthorized`: Authentication required
- `500 Internal Server Error`: Server error