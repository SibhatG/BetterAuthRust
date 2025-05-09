# Better Auth Integration Guide

This guide explains how to integrate and utilize the advanced authentication features provided by the Better Auth system in your application.

## Table of Contents

1. [Basic Authentication](#basic-authentication)
2. [WebAuthn Passwordless Authentication](#webauthn-passwordless-authentication)
3. [AI-based Risk Scoring](#ai-based-risk-scoring)
4. [Breach Detection](#breach-detection)
5. [Proxy Email Service](#proxy-email-service)
6. [Hybrid Encryption (RSA + CRYSTALS-Kyber)](#hybrid-encryption)
7. [Accessibility Features](#accessibility-features)
8. [HIPAA Compliance](#hipaa-compliance)
9. [TypeScript Client Integration](#typescript-client-integration)

## Basic Authentication

### Registration

```typescript
// Client-side
import { BetterAuthClient } from 'better-auth-client';

const auth = new BetterAuthClient('https://your-api-endpoint.com');

// Register a new user
async function registerUser(username, email, password) {
  try {
    const response = await auth.auth.register({
      username,
      email,
      password,
      password_confirmation: password
    });
    
    console.log('User registered:', response.user);
    return response;
  } catch (error) {
    console.error('Registration failed:', error);
    throw error;
  }
}
```

### Login

```typescript
// Client-side login with username/email and password
async function loginUser(usernameOrEmail, password) {
  try {
    const response = await auth.auth.login({
      username_or_email: usernameOrEmail,
      password
    });
    
    // Store the token for future authenticated requests
    auth.setAuthToken(response.access_token);
    
    console.log('User logged in:', response.user);
    return response;
  } catch (error) {
    console.error('Login failed:', error);
    throw error;
  }
}
```

### Authenticated Requests

```typescript
// Make an authenticated request
async function getCurrentUser() {
  try {
    const user = await auth.auth.getCurrentUser();
    console.log('Current user:', user);
    return user;
  } catch (error) {
    console.error('Failed to get user:', error);
    throw error;
  }
}
```

## WebAuthn Passwordless Authentication

WebAuthn enables passwordless authentication using biometrics or security keys.

### Registration Process

```typescript
// Start WebAuthn registration
async function startWebAuthnRegistration() {
  try {
    const challenge = await auth.auth.startWebAuthnRegistration();
    
    // Convert base64 challenge to Uint8Array
    const challengeBuffer = _base64ToArrayBuffer(challenge.options.challenge);
    
    // Create PublicKeyCredentialCreationOptions
    const options = {
      challenge: challengeBuffer,
      rp: {
        id: challenge.options.rp_id,
        name: challenge.options.rp_name
      },
      user: {
        id: _base64ToArrayBuffer(challenge.options.user_id),
        name: challenge.options.username,
        displayName: challenge.options.username
      },
      pubKeyCredParams: [
        { type: 'public-key', alg: -7 }, // ES256
        { type: 'public-key', alg: -257 } // RS256
      ],
      timeout: challenge.options.timeout,
      attestation: 'direct'
    };
    
    // Create credentials using the browser's WebAuthn API
    const credential = await navigator.credentials.create({
      publicKey: options
    });
    
    // Complete registration with the server
    const result = await auth.auth.completeWebAuthnRegistration({
      registration_id: challenge.registration_id,
      credential: _credentialToJson(credential)
    });
    
    console.log('WebAuthn registration complete:', result);
    return result;
  } catch (error) {
    console.error('WebAuthn registration failed:', error);
    throw error;
  }
}

// Convert credential to JSON-serializable object
function _credentialToJson(credential) {
  return {
    id: credential.id,
    raw_id: _arrayBufferToBase64(credential.rawId),
    response: {
      client_data_json: _arrayBufferToBase64(credential.response.clientDataJSON),
      attestation_object: _arrayBufferToBase64(credential.response.attestationObject)
    },
    type: credential.type
  };
}
```

### Authentication Process

```typescript
// Start WebAuthn login
async function startWebAuthnLogin(usernameOrEmail) {
  try {
    const challenge = await auth.auth.startWebAuthnLogin({
      username_or_email: usernameOrEmail
    });
    
    // Convert base64 challenge to Uint8Array
    const challengeBuffer = _base64ToArrayBuffer(challenge.options.challenge);
    
    // Create PublicKeyCredentialRequestOptions
    const options = {
      challenge: challengeBuffer,
      rpId: challenge.options.rp_id,
      timeout: challenge.options.timeout,
      userVerification: 'preferred'
    };
    
    // Get credentials using the browser's WebAuthn API
    const credential = await navigator.credentials.get({
      publicKey: options
    });
    
    // Complete authentication with the server
    const result = await auth.auth.completeWebAuthnLogin({
      authentication_id: challenge.authentication_id,
      credential: _authCredentialToJson(credential)
    });
    
    // Set the auth token
    auth.setAuthToken(result.access_token);
    
    console.log('WebAuthn login complete:', result);
    return result;
  } catch (error) {
    console.error('WebAuthn login failed:', error);
    throw error;
  }
}

// Convert authentication credential to JSON-serializable object
function _authCredentialToJson(credential) {
  return {
    id: credential.id,
    raw_id: _arrayBufferToBase64(credential.rawId),
    response: {
      client_data_json: _arrayBufferToBase64(credential.response.clientDataJSON),
      authenticator_data: _arrayBufferToBase64(credential.response.authenticatorData),
      signature: _arrayBufferToBase64(credential.response.signature),
      user_handle: credential.response.userHandle ? 
        _arrayBufferToBase64(credential.response.userHandle) : null
    },
    type: credential.type
  };
}
```

## AI-based Risk Scoring

The risk scoring system analyzes login patterns to detect suspicious activities.

### Handling Risk Scores

```typescript
// When a user logs in, check the risk score
async function handleRiskScore(loginResponse) {
  if (loginResponse.risk_analysis) {
    const { score, action, factors } = loginResponse.risk_analysis;
    
    if (action === 'Block') {
      // Deny login and show warning
      console.error('Login blocked due to security concerns');
      alert('Login blocked: Suspicious activity detected. Please contact support.');
      return false;
    } else if (action === 'RequireMfa') {
      // Require additional MFA verification
      return promptForMfa(loginResponse.user.id);
    }
  }
  
  return true; // Allow login to proceed
}

// Display risk factors to user if available
function showRiskFactors(factors) {
  if (factors && factors.length > 0) {
    console.warn('Risk factors detected:', factors);
    
    // Show warning to user
    const factorList = factors.map(f => `- ${f.description} (Risk: ${f.weight}%)`).join('\n');
    alert(`Login from unusual context detected:\n${factorList}\n\nFor security, additional verification required.`);
  }
}
```

## Breach Detection

The breach detection system identifies compromised credentials.

### Checking for Breaches

```typescript
// Check if user's credentials have been compromised
async function checkBreachStatus() {
  try {
    const result = await auth.breachDetection.checkBreachStatus();
    
    if (result.is_breached) {
      console.warn('Security breach detected!', result);
      
      if (result.password_compromised) {
        // Force password reset
        alert('Your password has been found in a data breach. Please reset your password immediately.');
        redirectToPasswordReset();
      } else if (result.breaches.length > 0) {
        // Notify about the breach
        const breachInfo = result.breaches.map(b => 
          `${b.source} (${b.breach_date}): ${b.data_types.join(', ')}`
        ).join('\n');
        
        alert(`Your email was found in the following data breaches:\n${breachInfo}\n\nConsider updating your security settings.`);
      }
      
      return result;
    }
    
    console.log('Breach check completed, no issues found');
    return result;
  } catch (error) {
    console.error('Breach check failed:', error);
    throw error;
  }
}
```

## Proxy Email Service

The proxy email service creates disposable email addresses to protect users' real email addresses.

### Managing Proxy Emails

```typescript
// Create a new proxy email
async function createProxyEmail(label) {
  try {
    const proxyEmail = await auth.proxyEmail.createProxyEmail({
      label: label || 'Default'
    });
    
    console.log('New proxy email created:', proxyEmail);
    return proxyEmail;
  } catch (error) {
    console.error('Failed to create proxy email:', error);
    throw error;
  }
}

// List all proxy emails
async function listProxyEmails() {
  try {
    const response = await auth.proxyEmail.listProxyEmails();
    console.log('Proxy emails:', response.proxy_emails);
    return response.proxy_emails;
  } catch (error) {
    console.error('Failed to list proxy emails:', error);
    throw error;
  }
}

// Update proxy email status (enable/disable)
async function updateProxyEmailStatus(proxyAddress, isEnabled) {
  try {
    const result = await auth.proxyEmail.updateProxyEmailStatus({
      proxy_address: proxyAddress,
      status: isEnabled ? 'Active' : 'Disabled'
    });
    
    console.log('Proxy email status updated:', result);
    return result;
  } catch (error) {
    console.error('Failed to update proxy email status:', error);
    throw error;
  }
}

// Delete a proxy email
async function deleteProxyEmail(proxyAddress) {
  try {
    await auth.proxyEmail.deleteProxyEmail(proxyAddress);
    console.log('Proxy email deleted:', proxyAddress);
    return true;
  } catch (error) {
    console.error('Failed to delete proxy email:', error);
    throw error;
  }
}
```

## Hybrid Encryption

The hybrid encryption system combines RSA and post-quantum CRYSTALS-Kyber for secure data protection.

### Using Hybrid Encryption

```typescript
// Get the user's public keys
async function getPublicKeys() {
  try {
    const keyPair = await auth.hybridEncryption.getPublicKeys();
    console.log('Public keys retrieved:', keyPair);
    return keyPair;
  } catch (error) {
    console.error('Failed to get public keys:', error);
    throw error;
  }
}

// Encrypt sensitive data
async function encryptData(data) {
  try {
    const encrypted = await auth.hybridEncryption.encrypt({
      data: JSON.stringify(data)
    });
    
    console.log('Data encrypted successfully');
    return encrypted;
  } catch (error) {
    console.error('Encryption failed:', error);
    throw error;
  }
}

// Decrypt sensitive data
async function decryptData(encryptedData) {
  try {
    const decrypted = await auth.hybridEncryption.decrypt({
      encrypted_data: encryptedData
    });
    
    console.log('Data decrypted successfully');
    return JSON.parse(decrypted.data);
  } catch (error) {
    console.error('Decryption failed:', error);
    throw error;
  }
}

// Rotate encryption keys
async function rotateKeys() {
  try {
    const result = await auth.hybridEncryption.rotateKeys();
    console.log('Encryption keys rotated successfully:', result);
    return result;
  } catch (error) {
    console.error('Key rotation failed:', error);
    throw error;
  }
}
```

## Accessibility Features

Enhance your authentication process with accessibility features to ensure all users can access your application.

### Managing Accessibility Preferences

```typescript
// Get accessibility preferences
async function getAccessibilityPreferences() {
  try {
    const preferences = await auth.accessibility.getPreferences();
    console.log('Accessibility preferences:', preferences);
    return preferences;
  } catch (error) {
    console.error('Failed to get accessibility preferences:', error);
    throw error;
  }
}

// Update accessibility preferences
async function updateAccessibilityPreferences(preferences) {
  try {
    const result = await auth.accessibility.updatePreferences(preferences);
    console.log('Accessibility preferences updated:', result);
    
    // Apply CSS variables based on new preferences
    applyAccessibilityCss();
    
    return result;
  } catch (error) {
    console.error('Failed to update accessibility preferences:', error);
    throw error;
  }
}

// Get CSS variables for accessibility
async function applyAccessibilityCss() {
  try {
    const cssVariables = await auth.accessibility.getCssVariables();
    
    // Add CSS variables to :root
    const style = document.createElement('style');
    style.textContent = `:root {${cssVariables}}`;
    
    // Remove old style if exists
    const oldStyle = document.getElementById('accessibility-styles');
    if (oldStyle) {
      oldStyle.remove();
    }
    
    // Add new style
    style.id = 'accessibility-styles';
    document.head.appendChild(style);
    
    console.log('Accessibility CSS applied');
  } catch (error) {
    console.error('Failed to apply accessibility CSS:', error);
    throw error;
  }
}

// Get accessible CAPTCHA alternative based on user preferences
async function getAccessibleCaptcha() {
  try {
    const captchaType = await auth.accessibility.getCaptchaAlternative();
    console.log('Using CAPTCHA type:', captchaType);
    return captchaType;
  } catch (error) {
    console.error('Failed to get CAPTCHA alternative:', error);
    throw error;
  }
}

// Process voice commands for voice-enabled login
async function processVoiceCommand(audioData) {
  try {
    const command = await auth.accessibility.processVoiceCommand(audioData);
    console.log('Voice command processed:', command);
    
    // Execute the voice command
    if (command.confidence > 0.8) {
      executeVoiceAction(command.action);
    } else {
      console.warn('Voice command confidence too low:', command.confidence);
    }
    
    return command;
  } catch (error) {
    console.error('Failed to process voice command:', error);
    throw error;
  }
}
```

## HIPAA Compliance

Implement HIPAA-compliant authentication controls for healthcare applications.

### Managing HIPAA Compliance

```typescript
// Get user role for HIPAA compliance
async function getUserRole() {
  try {
    const role = await auth.hipaaCompliance.getUserRole();
    console.log('User role:', role);
    return role;
  } catch (error) {
    console.error('Failed to get user role:', error);
    throw error;
  }
}

// Get permissions for the current role
async function getRolePermissions() {
  try {
    const permissions = await auth.hipaaCompliance.getRolePermissions();
    console.log('Role permissions:', permissions);
    return permissions;
  } catch (error) {
    console.error('Failed to get role permissions:', error);
    throw error;
  }
}

// Check if user has permission to access a resource
async function checkPermission(resourceType, accessType) {
  try {
    const hasPermission = await auth.hipaaCompliance.checkPermission(resourceType, accessType);
    console.log(`Permission to ${accessType} ${resourceType}: ${hasPermission}`);
    return hasPermission;
  } catch (error) {
    console.error('Permission check failed:', error);
    throw error;
  }
}

// Get active sessions for the current user
async function getActiveSessions() {
  try {
    const sessions = await auth.hipaaCompliance.getActiveSessions();
    console.log('Active sessions:', sessions);
    return sessions;
  } catch (error) {
    console.error('Failed to get active sessions:', error);
    throw error;
  }
}

// Terminate a session
async function terminateSession(sessionId) {
  try {
    const result = await auth.hipaaCompliance.terminateSession(sessionId);
    console.log('Session terminated:', result);
    return result;
  } catch (error) {
    console.error('Failed to terminate session:', error);
    throw error;
  }
}

// Get access logs for audit purposes
async function getAccessLogs(startDate, endDate) {
  try {
    const logs = await auth.hipaaCompliance.getAccessLogs(startDate, endDate);
    console.log('Access logs:', logs);
    return logs;
  } catch (error) {
    console.error('Failed to get access logs:', error);
    throw error;
  }
}

// Log emergency access
async function logEmergencyAccess(reason, resources) {
  try {
    const result = await auth.hipaaCompliance.logEmergencyAccess({
      reason,
      resources
    });
    console.log('Emergency access logged:', result);
    return result;
  } catch (error) {
    console.error('Failed to log emergency access:', error);
    throw error;
  }
}

// Generate audit report
async function generateAuditReport(startDate, endDate) {
  try {
    const reportUrl = await auth.hipaaCompliance.generateAuditReport({
      start_date: startDate,
      end_date: endDate
    });
    console.log('Audit report generated:', reportUrl);
    return reportUrl;
  } catch (error) {
    console.error('Failed to generate audit report:', error);
    throw error;
  }
}
```

## TypeScript Client Integration

To use the Better Auth client in your TypeScript application:

### Installation

```bash
npm install better-auth-client
```

### Basic Setup

```typescript
import { BetterAuthClient } from 'better-auth-client';

// Initialize the client
const auth = new BetterAuthClient('https://your-api-endpoint.com');

// Optional: Set an existing auth token (if resuming a session)
auth.setAuthToken(existingToken);
```

### Services Available

The client provides access to all Better Auth services:

```typescript
// Authentication service
auth.auth

// Risk scoring service
auth.riskScoring

// Breach detection service
auth.breachDetection

// Proxy email service
auth.proxyEmail

// Accessibility service
auth.accessibility

// HIPAA compliance service
auth.hipaaCompliance

// Hybrid encryption service
auth.hybridEncryption
```

### Example: Complete User Authentication Flow

```typescript
async function setupAuthFlow(app) {
  // Register login handler
  app.on('login-form-submit', async ({ username, password }) => {
    try {
      // Attempt login
      const loginResponse = await auth.auth.login({
        username_or_email: username,
        password
      });
      
      // Check risk score
      if (loginResponse.risk_analysis && 
          loginResponse.risk_analysis.action !== 'Allow') {
        if (loginResponse.risk_analysis.action === 'Block') {
          app.showError('Login blocked due to security concerns');
          return;
        } else {
          // Require MFA
          app.showMfaPrompt(loginResponse.user);
          return;
        }
      }
      
      // Check breach status
      const breachStatus = await auth.breachDetection.checkBreachStatus();
      if (breachStatus.is_breached && breachStatus.password_compromised) {
        app.redirectToPasswordReset();
        return;
      }
      
      // Login successful
      auth.setAuthToken(loginResponse.access_token);
      app.setCurrentUser(loginResponse.user);
      app.navigate('dashboard');
      
    } catch (error) {
      app.showError('Login failed: ' + error.message);
    }
  });
  
  // Add WebAuthn support
  if (window.PublicKeyCredential) {
    app.enableWebAuthnLogin();
    app.on('webauthn-login', async (username) => {
      try {
        const challenge = await auth.auth.startWebAuthnLogin({
          username_or_email: username
        });
        
        // Continuation of WebAuthn login flow...
      } catch (error) {
        app.showError('WebAuthn login failed: ' + error.message);
      }
    });
  }
}
```

## Helper Functions

These utility functions are used by the examples above:

```typescript
// Convert base64 string to ArrayBuffer
function _base64ToArrayBuffer(base64) {
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) {
    bytes[i] = binary.charCodeAt(i);
  }
  return bytes.buffer;
}

// Convert ArrayBuffer to base64 string
function _arrayBufferToBase64(buffer) {
  const bytes = new Uint8Array(buffer);
  let binary = '';
  for (let i = 0; i < bytes.byteLength; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return btoa(binary);
}
```