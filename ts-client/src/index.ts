/**
 * BetterAuth - A comprehensive authentication system client
 */

import { ApiClient } from './api/api-client';
import { PasswordlessAuth } from './passwordless-auth';
import { accessibilityUtils } from './accessibility-utils';

/**
 * Main client for the BetterAuth system
 */
export class BetterAuthClient {
  private readonly apiClient: ApiClient;

  public readonly passwordless: PasswordlessAuth;

  /**
   * Creates a new BetterAuth client
   * 
   * @param baseURL The base URL of the BetterAuth API
   */
  constructor(baseURL: string) {
    this.apiClient = new ApiClient(baseURL);
    this.passwordless = new PasswordlessAuth(this.apiClient);
  }

  /**
   * Set the authentication token manually (if needed)
   * 
   * @param token The JWT token
   */
  public setAuthToken(token: string): void {
    this.apiClient.setAuthToken(token);
  }

  /**
   * Clear the authentication token
   */
  public clearAuthToken(): void {
    this.apiClient.clearAuthToken();
  }

  /**
   * Get accessibility utilities
   */
  public getAccessibilityUtils() {
    return accessibilityUtils;
  }

  /**
   * Apply accessibility preferences
   */
  public applyAccessibilityPreferences() {
    accessibilityUtils.applyPreferences();
  }

  /**
   * Create an accessible passwordless login form
   */
  public createAccessiblePasswordlessLoginForm(
    containerId: string,
    onSuccess: (message: string) => void,
    onError: (error: Error) => void
  ): void {
    this.passwordless.createAccessibleLoginForm(containerId, onSuccess, onError);
  }
}

// Export main class
export { accessibilityUtils };
export * from './types/accessibility';

// For browsers, add to window object
if (typeof window !== 'undefined') {
  (window as any).BetterAuth = {
    createClient: (baseUrl: string) => new BetterAuthClient(baseUrl),
    accessibilityUtils
  };
}