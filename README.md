# Better Auth - Rust Authentication System

A comprehensive authentication system built in Rust that provides secure user registration, login, session management, and access control.

## Features

- User Registration and Login
- Secure Password Handling
- Session Management with Refresh Tokens
- JSON Web Token (JWT) Authentication
- Email Verification
- Password Reset
- Multi-Factor Authentication (MFA)
- TOTP-based 2FA
- Recovery Codes for MFA
- Customizable Rate Limiting
- CORS Support

## Project Structure

```
better-auth-rust/
├── src/
│   ├── auth_types.rs      # Data structures and types
│   ├── auth_utils.rs      # Utility functions
│   ├── main.rs            # Application entry point
│   ├── db/                # Database connection handlers
│   ├── middleware/        # Auth middleware
│   ├── models/            # Data models
│   ├── routes/            # API routes
│   ├── services/          # Business logic
│   └── utils/             # Utility functions
├── tests/                 # Unit and integration tests
└── docs/                  # Documentation
```

## API Endpoints

### Authentication

- **POST /api/auth/register** - Register a new user
- **POST /api/auth/login** - Login with username/email and password
- **POST /api/auth/mfa-login** - Login with MFA code
- **POST /api/auth/refresh-token** - Refresh access token
- **POST /api/auth/logout** - Logout (invalidate session)
- **POST /api/auth/logout-all** - Logout from all devices

### Email Verification

- **POST /api/auth/verify-email** - Verify email with token
- **POST /api/auth/resend-verification-email** - Resend verification email

### Password Management

- **POST /api/auth/password-reset** - Request password reset
- **POST /api/auth/password-reset-confirm** - Reset password with token

### Multi-Factor Authentication

- **GET /api/auth/mfa-setup** - Setup MFA
- **POST /api/auth/mfa-enable** - Enable MFA
- **POST /api/auth/mfa-disable** - Disable MFA
- **GET /api/auth/mfa-recovery-codes** - Get MFA recovery codes
- **POST /api/auth/mfa-recovery** - MFA recovery with code

### User Information

- **GET /api/users/me** - Get current user information
- **GET /api/users/sessions** - List active sessions

## Getting Started

1. Install Rust and Cargo
2. Clone this repository
3. Configure environment variables in `.env` file
4. Run PostgreSQL database
5. Run the application:

```bash
cargo run
```

## Running Tests

```bash
cargo test
```

## Environment Variables

Create a `.env` file in the root directory with the following variables:

```
# Server configuration
SERVER_ADDR=0.0.0.0
SERVER_PORT=5000
SECRET_KEY=your_secret_key
ACCESS_TOKEN_EXPIRY=3600
REFRESH_TOKEN_EXPIRY=604800

# Database configuration
DATABASE_URL=postgres://username:password@localhost/auth_db
DATABASE_POOL_SIZE=5

# Email configuration
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USERNAME=your_username
SMTP_PASSWORD=your_password
EMAIL_FROM=no-reply@example.com

# Rate limiting
RATE_LIMIT_REQUESTS=100
RATE_LIMIT_DURATION=60
```

## License

MIT License