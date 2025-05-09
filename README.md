# Better Auth TypeScript Client

A comprehensive authentication system client for the Better Auth Rust backend. This client provides a complete set of APIs to interact with all features of the Better Auth system.

## Features

- **User Authentication**: Registration, login, and session management
- **FIDO2/WebAuthn**: Passwordless authentication with security keys and biometrics
- **Risk Scoring**: AI-based detection of suspicious login attempts
- **Breach Detection**: Automatic detection of compromised credentials
- **Proxy Email**: "Hide My Email" style email aliases
- **Hybrid Encryption**: Post-quantum secure encryption with RSA + CRYSTALS-Kyber
- **Accessibility**: Support for various accessibility needs
- **HIPAA Compliance**: Tools for healthcare applications

## Installation

```bash
npm install better-auth-client
```

## Usage

### Basic Authentication

```typescript
import BetterAuthClient from 'better-auth-client';

// Initialize the client
const auth = new BetterAuthClient('https://api.example.com');

// Register a new user
const register = async () => {
  try {
    const result = await auth.auth.register({
      username: 'johndoe',
      email: 'john@example.com',
      password: 'securePassword123',
      password_confirmation: 'securePassword123'
    });
    console.log('User registered successfully:', result);
  } catch (error) {
    console.error('Registration failed:', error);
  }
};

// Login
const login = async () => {
  try {
    const result = await auth.auth.login({
      username_or_email: 'johndoe',
      password: 'securePassword123'
    });
    console.log('Login successful:', result);
    // Token is automatically set for future requests
  } catch (error) {
    console.error('Login failed:', error);
  }
};

// Get current user
const getCurrentUser = async () => {
  try {
    const user = await auth.auth.getCurrentUser();
    console.log('Current user:', user);
  } catch (error) {
    console.error('Failed to get user:', error);
  }
};

// Logout
const logout = () => {
  auth.auth.logout();
  console.log('Logged out successfully');
};
```

### WebAuthn Passwordless Authentication

```typescript
// Start WebAuthn registration
const startWebAuthnRegistration = async () => {
  try {
    const options = await auth.auth.startWebAuthnRegistration();
    
    // Convert options to proper WebAuthn format and create credentials
    const credential = await navigator.credentials.create({
      publicKey: convertToPublicKeyCredentialCreationOptions(options)
    });
    
    // Complete registration
    const result = await auth.auth.completeWebAuthnRegistration({
      registration_id: options.registration_id,
      credential: convertFromCredential(credential)
    });
    
    console.log('WebAuthn registration successful:', result);
  } catch (error) {
    console.error('WebAuthn registration failed:', error);
  }
};

// Start WebAuthn login
const startWebAuthnLogin = async () => {
  try {
    const options = await auth.auth.startWebAuthnLogin({
      username_or_email: 'johndoe'
    });
    
    // Convert options to proper WebAuthn format and get credentials
    const credential = await navigator.credentials.get({
      publicKey: convertToPublicKeyCredentialRequestOptions(options)
    });
    
    // Complete login
    const result = await auth.auth.completeWebAuthnLogin({
      authentication_id: options.authentication_id,
      credential: convertFromCredential(credential)
    });
    
    console.log('WebAuthn login successful:', result);
  } catch (error) {
    console.error('WebAuthn login failed:', error);
  }
};
```

### Risk Scoring

```typescript
// Get risk analysis
const getRiskAnalysis = async () => {
  try {
    const result = await auth.riskScoring.getRiskAnalysis();
    
    console.log('Risk score:', result.risk_score);
    console.log('Risk factors:', result.risk_factors);
    console.log('Recommended action:', result.recommended_action);
    
    // Handle different risk levels
    if (result.recommended_action === 'RequireMfa') {
      // Prompt user for MFA
    } else if (result.recommended_action === 'Block') {
      // Block the login attempt
    }
  } catch (error) {
    console.error('Failed to get risk analysis:', error);
  }
};
```

### Proxy Email

```typescript
// Create a new proxy email
const createProxyEmail = async () => {
  try {
    const result = await auth.proxyEmail.createProxyEmail({
      label: 'Shopping Site'
    });
    
    console.log('New proxy email created:', result.proxy_address);
  } catch (error) {
    console.error('Failed to create proxy email:', error);
  }
};

// List all proxy emails
const listProxyEmails = async () => {
  try {
    const result = await auth.proxyEmail.listProxyEmails();
    
    console.log('Proxy emails:', result.proxy_emails);
  } catch (error) {
    console.error('Failed to list proxy emails:', error);
  }
};
```

## Advanced Configuration

### Setting Authentication Token Manually

```typescript
// If you have a token from elsewhere (e.g., from local storage)
auth.setAuthToken('your-jwt-token');
```

### Error Handling

The client automatically handles common error cases, but you can also catch and handle specific errors:

```typescript
try {
  await auth.auth.login({ username_or_email: 'johndoe', password: 'wrong' });
} catch (error) {
  if (error.response && error.response.status === 401) {
    console.error('Invalid credentials');
  } else if (error.response && error.response.status === 429) {
    console.error('Too many attempts, please try again later');
  } else {
    console.error('Unknown error:', error);
  }
}
```

## License

MIT