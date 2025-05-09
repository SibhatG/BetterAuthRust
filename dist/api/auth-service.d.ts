/**
 * Auth service for handling authentication operations
 */
import { ApiClient } from './api-client';
import { LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, WebAuthnLoginStartRequest, WebAuthnRegisterStartResponse, WebAuthnAuthenticateStartResponse, WebAuthnRegisterCompleteRequest, WebAuthnAuthenticateCompleteRequest, User } from '../types';
export declare class AuthService {
    private readonly apiClient;
    constructor(apiClient: ApiClient);
    /**
     * Register a new user
     */
    register(request: RegisterRequest): Promise<RegisterResponse>;
    /**
     * Login a user
     */
    login(request: LoginRequest): Promise<LoginResponse>;
    /**
     * Get the current user
     */
    getCurrentUser(): Promise<User>;
    /**
     * Logout the current user
     */
    logout(): void;
    /**
     * Start WebAuthn registration
     */
    startWebAuthnRegistration(): Promise<WebAuthnRegisterStartResponse>;
    /**
     * Complete WebAuthn registration
     */
    completeWebAuthnRegistration(request: WebAuthnRegisterCompleteRequest): Promise<{
        status: string;
        message: string;
    }>;
    /**
     * Start WebAuthn login
     */
    startWebAuthnLogin(request: WebAuthnLoginStartRequest): Promise<WebAuthnAuthenticateStartResponse>;
    /**
     * Complete WebAuthn login
     */
    completeWebAuthnLogin(request: WebAuthnAuthenticateCompleteRequest): Promise<LoginResponse>;
}
