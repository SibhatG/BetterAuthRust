# Better Auth Rust - API Documentation

This document provides comprehensive documentation for the Better Auth Rust authentication system API.

## Base URL

All API endpoints are relative to: `http://localhost:5000/api/`

## Authentication

Most endpoints require authentication using a Bearer token. To authenticate requests, include an `Authorization` header with a JWT token:

```
Authorization: Bearer <your_access_token>
```

## Endpoints

### User Registration and Authentication

#### Register a new user

- **URL**: `/auth/register`
- **Method**: `POST`
- **Auth required**: No
- **Request body**:
  ```json
  {
    "username": "johndoe",
    "email": "johndoe@example.com",
    "password": "securepassword",
    "password_confirmation": "securepassword"
  }
  ```
- **Success Response**:
  - **Code**: 201 Created
  - **Content**:
    ```json
    {
      "user": {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "username": "johndoe",
        "email": "johndoe@example.com",
        "is_email_verified": false,
        "mfa_enabled": false,
        "created_at": "2023-01-01T00:00:00Z",
        "updated_at": "2023-01-01T00:00:00Z",
        "last_login_at": null,
        "is_active": true,
        "is_admin": false
      },
      "message": "User registered successfully. Please check your email to verify your account."
    }
    ```

#### Login

- **URL**: `/auth/login`
- **Method**: `POST`
- **Auth required**: No
- **Request body**:
  ```json
  {
    "username_or_email": "johndoe@example.com",
    "password": "securepassword"
  }
  ```
- **Success Response**:
  - **Code**: 200 OK
  - **Content**:
    ```json
    {
      "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
      "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
      "token_type": "Bearer",
      "expires_in": 3600,
      "user": {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "username": "johndoe",
        "email": "johndoe@example.com",
        "is_email_verified": true,
        "mfa_enabled": false,
        "created_at": "2023-01-01T00:00:00Z",
        "updated_at": "2023-01-01T00:00:00Z",
        "last_login_at": "2023-01-01T00:00:00Z",
        "is_active": true,
        "is_admin": false
      },
      "mfa_required": false
    }
    ```

#### MFA Login (when MFA is enabled)

- **URL**: `/auth/mfa-login`
- **Method**: `POST`
- **Auth required**: No
- **Request body**:
  ```json
  {
    "username_or_email": "johndoe@example.com",
    "password": "securepassword",
    "mfa_code": "123456"
  }
  ```
- **Success Response**: Same as regular login

#### Refresh Token

- **URL**: `/auth/refresh-token`
- **Method**: `POST`
- **Auth required**: No
- **Request body**:
  ```json
  {
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
  ```
- **Success Response**:
  - **Code**: 200 OK
  - **Content**:
    ```json
    {
      "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
      "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
      "token_type": "Bearer",
      "expires_in": 3600
    }
    ```

#### Logout

- **URL**: `/auth/logout`
- **Method**: `POST`
- **Auth required**: Yes
- **Request body**:
  ```json
  {
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
  ```
- **Success Response**:
  - **Code**: 200 OK
  - **Content**:
    ```json
    {
      "message": "Successfully logged out"
    }
    ```

#### Logout from all devices

- **URL**: `/auth/logout-all`
- **Method**: `POST`
- **Auth required**: Yes
- **Success Response**:
  - **Code**: 200 OK
  - **Content**:
    ```json
    {
      "message": "Successfully logged out from all devices"
    }
    ```

### Email Verification

#### Verify Email

- **URL**: `/auth/verify-email`
- **Method**: `POST`
- **Auth required**: No
- **Request body**:
  ```json
  {
    "token": "verification-token"
  }
  ```
- **Success Response**:
  - **Code**: 200 OK
  - **Content**:
    ```json
    {
      "message": "Email verified successfully"
    }
    ```

#### Resend Verification Email

- **URL**: `/auth/resend-verification-email`
- **Method**: `POST`
- **Auth required**: Yes
- **Success Response**:
  - **Code**: 200 OK
  - **Content**:
    ```json
    {
      "message": "Verification email sent successfully"
    }
    ```

### Password Reset

#### Request Password Reset

- **URL**: `/auth/password-reset`
- **Method**: `POST`
- **Auth required**: No
- **Request body**:
  ```json
  {
    "email": "johndoe@example.com"
  }
  ```
- **Success Response**:
  - **Code**: 200 OK
  - **Content**:
    ```json
    {
      "message": "If your email is registered, you will receive a password reset link"
    }
    ```

#### Confirm Password Reset

- **URL**: `/auth/password-reset-confirm`
- **Method**: `POST`
- **Auth required**: No
- **Request body**:
  ```json
  {
    "token": "reset-token",
    "password": "newpassword",
    "password_confirmation": "newpassword"
  }
  ```
- **Success Response**:
  - **Code**: 200 OK
  - **Content**:
    ```json
    {
      "message": "Password reset successfully"
    }
    ```

### Multi-Factor Authentication (MFA)

#### Setup MFA

- **URL**: `/auth/mfa-setup`
- **Method**: `GET`
- **Auth required**: Yes
- **Success Response**:
  - **Code**: 200 OK
  - **Content**:
    ```json
    {
      "secret": "JBSWY3DPEHPK3PXP",
      "qr_code_url": "otpauth://totp/BetterAuth:johndoe@example.com?secret=JBSWY3DPEHPK3PXP&issuer=BetterAuth"
    }
    ```

#### Enable MFA

- **URL**: `/auth/mfa-enable`
- **Method**: `POST`
- **Auth required**: Yes
- **Request body**:
  ```json
  {
    "mfa_code": "123456"
  }
  ```
- **Success Response**:
  - **Code**: 200 OK
  - **Content**:
    ```json
    {
      "message": "MFA enabled successfully",
      "recovery_codes": [
        "1234-5678-9012",
        "2345-6789-0123",
        "3456-7890-1234",
        "4567-8901-2345",
        "5678-9012-3456",
        "6789-0123-4567",
        "7890-1234-5678",
        "8901-2345-6789"
      ]
    }
    ```

#### Disable MFA

- **URL**: `/auth/mfa-disable`
- **Method**: `POST`
- **Auth required**: Yes
- **Request body**:
  ```json
  {
    "mfa_code": "123456",
    "password": "securepassword"
  }
  ```
- **Success Response**:
  - **Code**: 200 OK
  - **Content**:
    ```json
    {
      "message": "MFA disabled successfully"
    }
    ```

#### Get Recovery Codes

- **URL**: `/auth/mfa-recovery-codes`
- **Method**: `GET`
- **Auth required**: Yes
- **Success Response**:
  - **Code**: 200 OK
  - **Content**:
    ```json
    {
      "recovery_codes": [
        "1234-5678-9012",
        "2345-6789-0123",
        "3456-7890-1234",
        "4567-8901-2345",
        "5678-9012-3456",
        "6789-0123-4567",
        "7890-1234-5678",
        "8901-2345-6789"
      ]
    }
    ```

#### MFA Recovery

- **URL**: `/auth/mfa-recovery`
- **Method**: `POST`
- **Auth required**: No
- **Request body**:
  ```json
  {
    "recovery_code": "1234-5678-9012"
  }
  ```
- **Success Response**: Same as regular login

### User Management

#### Get Current User

- **URL**: `/users/me`
- **Method**: `GET`
- **Auth required**: Yes
- **Success Response**:
  - **Code**: 200 OK
  - **Content**:
    ```json
    {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "username": "johndoe",
      "email": "johndoe@example.com",
      "is_email_verified": true,
      "mfa_enabled": true,
      "created_at": "2023-01-01T00:00:00Z",
      "updated_at": "2023-01-01T00:00:00Z",
      "last_login_at": "2023-01-01T00:00:00Z",
      "is_active": true,
      "is_admin": false
    }
    ```

#### Get User Sessions

- **URL**: `/users/sessions`
- **Method**: `GET`
- **Auth required**: Yes
- **Success Response**:
  - **Code**: 200 OK
  - **Content**:
    ```json
    [
      {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
        "ip_address": "192.168.1.1",
        "created_at": "2023-01-01T00:00:00Z",
        "expires_at": "2023-01-08T00:00:00Z",
        "is_current": true
      },
      {
        "id": "223e4567-e89b-12d3-a456-426614174000",
        "user_agent": "Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1",
        "ip_address": "192.168.1.2",
        "created_at": "2023-01-02T00:00:00Z",
        "expires_at": "2023-01-09T00:00:00Z",
        "is_current": false
      }
    ]
    ```

## Error Responses

All endpoints return standardized error responses in the following format:

```json
{
  "status": "error",
  "code": "ERROR_CODE",
  "message": "A human-readable error message",
  "details": {}  // Optional additional error details
}
```

Common error codes:
- `VALIDATION_ERROR`: Request validation failed
- `AUTHENTICATION_ERROR`: Authentication failed
- `AUTHORIZATION_ERROR`: User is not authorized to perform the action
- `NOT_FOUND`: Requested resource not found
- `CONFLICT`: Resource conflict, e.g., username already exists
- `INTERNAL_ERROR`: Server internal error
