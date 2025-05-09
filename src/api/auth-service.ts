/**
 * Auth service for handling authentication operations
 */

import { ApiClient } from './api-client';
import {
  LoginRequest,
  LoginResponse,
  RegisterRequest,
  RegisterResponse,
  WebAuthnLoginStartRequest,
  WebAuthnRegisterStartResponse,
  WebAuthnAuthenticateStartResponse,
  WebAuthnRegisterCompleteRequest,
  WebAuthnAuthenticateCompleteRequest,
  User
} from '../types';

export class AuthService {
  private readonly apiClient: ApiClient;

  constructor(apiClient: ApiClient) {
    this.apiClient = apiClient;
  }

  /**
   * Register a new user
   */
  public async register(request: RegisterRequest): Promise<RegisterResponse> {
    return this.apiClient.post<RegisterResponse>('/api/auth/register', request);
  }

  /**
   * Login a user
   */
  public async login(request: LoginRequest): Promise<LoginResponse> {
    const response = await this.apiClient.post<LoginResponse>('/api/auth/login', request);
    
    // Set the auth token for subsequent requests
    this.apiClient.setAuthToken(response.access_token);
    
    return response;
  }

  /**
   * Get the current user
   */
  public async getCurrentUser(): Promise<User> {
    return this.apiClient.get<User>('/api/users/me');
  }

  /**
   * Logout the current user
   */
  public logout(): void {
    this.apiClient.clearAuthToken();
  }

  /**
   * Start WebAuthn registration
   */
  public async startWebAuthnRegistration(): Promise<WebAuthnRegisterStartResponse> {
    return this.apiClient.post<WebAuthnRegisterStartResponse>('/api/auth/webauthn/register/start');
  }

  /**
   * Complete WebAuthn registration
   */
  public async completeWebAuthnRegistration(
    request: WebAuthnRegisterCompleteRequest
  ): Promise<{ status: string; message: string }> {
    return this.apiClient.post<{ status: string; message: string }>(
      '/api/auth/webauthn/register/complete',
      request
    );
  }

  /**
   * Start WebAuthn login
   */
  public async startWebAuthnLogin(
    request: WebAuthnLoginStartRequest
  ): Promise<WebAuthnAuthenticateStartResponse> {
    return this.apiClient.post<WebAuthnAuthenticateStartResponse>(
      '/api/auth/webauthn/login/start',
      request
    );
  }

  /**
   * Complete WebAuthn login
   */
  public async completeWebAuthnLogin(
    request: WebAuthnAuthenticateCompleteRequest
  ): Promise<LoginResponse> {
    const response = await this.apiClient.post<LoginResponse>(
      '/api/auth/webauthn/login/complete',
      request
    );
    
    // Set the auth token for subsequent requests
    this.apiClient.setAuthToken(response.access_token);
    
    return response;
  }
}