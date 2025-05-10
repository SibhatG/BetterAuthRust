/**
 * WebAuthn-based passwordless authentication utilities
 */

import { ApiClient } from './api/api-client';
import { accessibilityUtils } from './accessibility-utils';

/**
 * Passwordless authentication using WebAuthn
 */
export class PasswordlessAuth {
  private readonly apiClient: ApiClient;

  /**
   * Constructs a new PasswordlessAuth instance
   * @param apiClient ApiClient instance for API communication
   */
  constructor(apiClient: ApiClient) {
    this.apiClient = apiClient;
  }

  /**
   * Check if WebAuthn is supported in the current browser
   */
  public isWebAuthnSupported(): boolean {
    return window.PublicKeyCredential !== undefined;
  }

  /**
   * Start the passwordless registration process
   * @param username User's username
   * @param email User's email
   * @param deviceName Optional device name
   */
  public async startPasswordlessRegistration(
    username: string,
    email: string,
    deviceName?: string
  ): Promise<string> {
    try {
      // Check WebAuthn support
      if (!this.isWebAuthnSupported()) {
        throw new Error('WebAuthn is not supported in this browser');
      }

      // Start registration on server
      const response = await this.apiClient.post('/auth/passwordless-register-start', {
        username,
        email,
        device_name: deviceName,
      });

      // Store registration ID
      const registrationId = response.registration_id;
      const options = response.options;

      // Convert options to PublicKeyCredentialCreationOptions
      const publicKeyOptions = this.convertRegistrationOptions(options);

      // Create credential
      const credential = await navigator.credentials.create({
        publicKey: publicKeyOptions,
      }) as PublicKeyCredential;

      // Complete registration
      await this.completePasswordlessRegistration(registrationId, credential);

      return 'Registration successful';
    } catch (error) {
      console.error('Passwordless registration error:', error);
      throw error;
    }
  }

  /**
   * Complete passwordless registration after user confirms with authenticator
   * @param registrationId Registration ID from server
   * @param credential WebAuthn credential from browser
   */
  private async completePasswordlessRegistration(
    registrationId: string,
    credential: PublicKeyCredential
  ): Promise<void> {
    // Prepare attestation response
    const attestationResponse = credential.response as AuthenticatorAttestationResponse;
    
    // Convert to base64 format for server
    const credentialResponse = {
      id: credential.id,
      raw_id: this.arrayBufferToBase64(credential.rawId),
      response: {
        client_data_json: this.arrayBufferToBase64(attestationResponse.clientDataJSON),
        attestation_object: this.arrayBufferToBase64(attestationResponse.attestationObject),
      },
      type: credential.type,
    };

    // Complete registration on server
    await this.apiClient.post('/auth/passwordless-register-complete', {
      registration_id: registrationId,
      credential: credentialResponse,
    });
  }

  /**
   * Start the passwordless login process
   * @param usernameOrEmail User's username or email
   */
  public async startPasswordlessLogin(usernameOrEmail: string): Promise<string> {
    try {
      // Check WebAuthn support
      if (!this.isWebAuthnSupported()) {
        throw new Error('WebAuthn is not supported in this browser');
      }

      // Start login on server
      const response = await this.apiClient.post('/auth/passwordless-login-start', {
        username_or_email: usernameOrEmail,
      });

      // Store authentication ID
      const authenticationId = response.authentication_id;
      const options = response.options;

      // Convert options to PublicKeyCredentialRequestOptions
      const publicKeyOptions = this.convertAuthenticationOptions(options);

      // Get credential
      const credential = await navigator.credentials.get({
        publicKey: publicKeyOptions,
      }) as PublicKeyCredential;

      // Complete login
      const loginResult = await this.completePasswordlessLogin(authenticationId, credential);

      // Set auth token
      this.apiClient.setAuthToken(loginResult.access_token);

      return 'Login successful';
    } catch (error) {
      console.error('Passwordless login error:', error);
      throw error;
    }
  }

  /**
   * Complete passwordless login after user confirms with authenticator
   * @param authenticationId Authentication ID from server
   * @param credential WebAuthn credential from browser
   */
  private async completePasswordlessLogin(
    authenticationId: string,
    credential: PublicKeyCredential
  ): Promise<any> {
    // Prepare assertion response
    const assertionResponse = credential.response as AuthenticatorAssertionResponse;
    
    // Convert to base64 format for server
    const credentialResponse = {
      id: credential.id,
      raw_id: this.arrayBufferToBase64(credential.rawId),
      response: {
        client_data_json: this.arrayBufferToBase64(assertionResponse.clientDataJSON),
        authenticator_data: this.arrayBufferToBase64(assertionResponse.authenticatorData),
        signature: this.arrayBufferToBase64(assertionResponse.signature),
        user_handle: assertionResponse.userHandle ? 
          this.arrayBufferToBase64(assertionResponse.userHandle) : null,
      },
      type: credential.type,
    };

    // Complete login on server
    return await this.apiClient.post('/auth/passwordless-login-complete', {
      authentication_id: authenticationId,
      credential: credentialResponse,
    });
  }

  /**
   * Convert WebAuthn registration options from server to browser format
   */
  private convertRegistrationOptions(options: any): PublicKeyCredentialCreationOptions {
    return {
      challenge: this.base64ToArrayBuffer(options.challenge),
      rp: {
        name: options.rp_name,
        id: options.rp_id,
      },
      user: {
        id: this.base64ToArrayBuffer(options.user_id),
        name: options.username,
        displayName: options.username,
      },
      pubKeyCredParams: [
        { type: 'public-key', alg: -7 }, // ES256
        { type: 'public-key', alg: -257 }, // RS256
      ],
      timeout: options.timeout,
      attestation: 'direct',
      authenticatorSelection: {
        userVerification: 'preferred',
        requireResidentKey: false,
      },
    };
  }

  /**
   * Convert WebAuthn authentication options from server to browser format
   */
  private convertAuthenticationOptions(options: any): PublicKeyCredentialRequestOptions {
    return {
      challenge: this.base64ToArrayBuffer(options.challenge),
      rpId: options.rp_id,
      timeout: options.timeout,
      userVerification: 'preferred',
    };
  }

  /**
   * Convert ArrayBuffer to Base64 string
   */
  private arrayBufferToBase64(buffer: ArrayBuffer): string {
    const bytes = new Uint8Array(buffer);
    let binary = '';
    for (let i = 0; i < bytes.byteLength; i++) {
      binary += String.fromCharCode(bytes[i]);
    }
    return btoa(binary);
  }

  /**
   * Convert Base64 string to ArrayBuffer
   */
  private base64ToArrayBuffer(base64: string): ArrayBuffer {
    const binaryString = atob(base64);
    const bytes = new Uint8Array(binaryString.length);
    for (let i = 0; i < binaryString.length; i++) {
      bytes[i] = binaryString.charCodeAt(i);
    }
    return bytes.buffer;
  }

  /**
   * Create an accessible passwordless login form
   * @param containerId ID of container element to insert form
   * @param onSuccess Success callback
   * @param onError Error callback
   */
  public createAccessibleLoginForm(
    containerId: string,
    onSuccess: (message: string) => void,
    onError: (error: Error) => void
  ): void {
    const container = document.getElementById(containerId);
    if (!container) {
      onError(new Error(`Container element with ID '${containerId}' not found`));
      return;
    }

    // Create form
    const form = document.createElement('div');
    form.className = 'better-auth-passwordless-form';
    form.setAttribute('role', 'form');
    form.setAttribute('aria-labelledby', 'passwordless-login-title');

    // Title with screen reader support
    const title = document.createElement('h2');
    title.id = 'passwordless-login-title';
    title.textContent = 'Passwordless Login';
    title.setAttribute('tabindex', '0');

    // Create username/email input
    const inputGroup = document.createElement('div');
    inputGroup.className = 'input-group';

    const label = document.createElement('label');
    label.setAttribute('for', 'username-or-email');
    label.textContent = 'Username or Email';

    const input = document.createElement('input');
    input.type = 'text';
    input.id = 'username-or-email';
    input.name = 'username-or-email';
    input.required = true;
    input.setAttribute('aria-required', 'true');
    input.placeholder = 'Enter your username or email';
    input.setAttribute('autocomplete', 'username webauthn');

    inputGroup.appendChild(label);
    inputGroup.appendChild(input);

    // Create login button
    const button = document.createElement('button');
    button.type = 'button';
    button.textContent = 'Login with Biometrics';
    button.setAttribute('aria-describedby', 'passwordless-description');
    button.className = 'passwordless-button';

    // Description for screen readers
    const description = document.createElement('div');
    description.id = 'passwordless-description';
    description.className = 'sr-only';
    description.textContent = 'This will use your fingerprint, face recognition, or security key to log in securely without a password.';

    // Status message area
    const status = document.createElement('div');
    status.className = 'status-message';
    status.setAttribute('role', 'status');
    status.setAttribute('aria-live', 'polite');

    // Assemble form
    form.appendChild(title);
    form.appendChild(inputGroup);
    form.appendChild(description);
    form.appendChild(button);
    form.appendChild(status);

    // Handle login
    button.addEventListener('click', async () => {
      const usernameOrEmail = input.value.trim();
      if (!usernameOrEmail) {
        status.textContent = 'Please enter your username or email';
        status.className = 'status-message error';
        return;
      }

      try {
        status.textContent = 'Authenticating...';
        status.className = 'status-message';
        
        const result = await this.startPasswordlessLogin(usernameOrEmail);
        
        status.textContent = result;
        status.className = 'status-message success';
        
        onSuccess(result);
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : 'Login failed';
        status.textContent = errorMessage;
        status.className = 'status-message error';
        onError(error instanceof Error ? error : new Error(errorMessage));
      }
    });

    // Add keyboard shortcuts if enabled
    if (accessibilityUtils.getPreferences().keyboard_navigation) {
      input.addEventListener('keydown', (event) => {
        if (event.key === 'Enter') {
          button.click();
          event.preventDefault();
        }
      });
    }

    // Style the form
    const style = document.createElement('style');
    style.textContent = `
      .better-auth-passwordless-form {
        font-family: system-ui, -apple-system, sans-serif;
        max-width: 400px;
        margin: 0 auto;
        padding: 20px;
        border-radius: 8px;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        background-color: #fff;
      }
      
      .better-auth-passwordless-form h2 {
        margin-top: 0;
        color: #333;
        text-align: center;
      }
      
      .better-auth-passwordless-form .input-group {
        margin-bottom: 20px;
      }
      
      .better-auth-passwordless-form label {
        display: block;
        margin-bottom: 8px;
        font-weight: 500;
      }
      
      .better-auth-passwordless-form input {
        width: 100%;
        padding: 10px;
        border: 1px solid #ddd;
        border-radius: 4px;
        font-size: 16px;
      }
      
      .better-auth-passwordless-form .passwordless-button {
        width: 100%;
        padding: 12px;
        background-color: #4285f4;
        color: white;
        border: none;
        border-radius: 4px;
        font-size: 16px;
        cursor: pointer;
        transition: background-color 0.3s;
      }
      
      .better-auth-passwordless-form .passwordless-button:hover {
        background-color: #3367d6;
      }
      
      .better-auth-passwordless-form .status-message {
        margin-top: 15px;
        padding: 10px;
        border-radius: 4px;
        text-align: center;
      }
      
      .better-auth-passwordless-form .status-message.error {
        background-color: #ffebee;
        color: #d32f2f;
      }
      
      .better-auth-passwordless-form .status-message.success {
        background-color: #e8f5e9;
        color: #388e3c;
      }
      
      .better-auth-passwordless-form .sr-only {
        position: absolute;
        width: 1px;
        height: 1px;
        padding: 0;
        margin: -1px;
        overflow: hidden;
        clip: rect(0, 0, 0, 0);
        white-space: nowrap;
        border-width: 0;
      }
      
      /* High contrast mode */
      .high-contrast .better-auth-passwordless-form {
        background-color: #000;
        color: #fff;
        border: 2px solid #fff;
      }
      
      .high-contrast .better-auth-passwordless-form h2 {
        color: #fff;
      }
      
      .high-contrast .better-auth-passwordless-form input {
        background-color: #333;
        color: #fff;
        border-color: #fff;
      }
      
      .high-contrast .better-auth-passwordless-form .passwordless-button {
        background-color: #ffff00;
        color: #000;
        border: 2px solid #fff;
      }
      
      /* Large text mode */
      .large-text .better-auth-passwordless-form {
        font-size: 18px;
      }
      
      .large-text .better-auth-passwordless-form h2 {
        font-size: 24px;
      }
      
      .large-text .better-auth-passwordless-form input {
        font-size: 18px;
        padding: 12px;
      }
      
      .large-text .better-auth-passwordless-form .passwordless-button {
        font-size: 18px;
        padding: 15px;
      }
    `;

    // Add form and styles to container
    document.head.appendChild(style);
    container.appendChild(form);
  }
}