"use strict";
/**
 * Auth service for handling authentication operations
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.AuthService = void 0;
class AuthService {
    constructor(apiClient) {
        this.apiClient = apiClient;
    }
    /**
     * Register a new user
     */
    async register(request) {
        return this.apiClient.post('/api/auth/register', request);
    }
    /**
     * Login a user
     */
    async login(request) {
        const response = await this.apiClient.post('/api/auth/login', request);
        // Set the auth token for subsequent requests
        this.apiClient.setAuthToken(response.access_token);
        return response;
    }
    /**
     * Get the current user
     */
    async getCurrentUser() {
        return this.apiClient.get('/api/users/me');
    }
    /**
     * Logout the current user
     */
    logout() {
        this.apiClient.clearAuthToken();
    }
    /**
     * Start WebAuthn registration
     */
    async startWebAuthnRegistration() {
        return this.apiClient.post('/api/auth/webauthn/register/start');
    }
    /**
     * Complete WebAuthn registration
     */
    async completeWebAuthnRegistration(request) {
        return this.apiClient.post('/api/auth/webauthn/register/complete', request);
    }
    /**
     * Start WebAuthn login
     */
    async startWebAuthnLogin(request) {
        return this.apiClient.post('/api/auth/webauthn/login/start', request);
    }
    /**
     * Complete WebAuthn login
     */
    async completeWebAuthnLogin(request) {
        const response = await this.apiClient.post('/api/auth/webauthn/login/complete', request);
        // Set the auth token for subsequent requests
        this.apiClient.setAuthToken(response.access_token);
        return response;
    }
}
exports.AuthService = AuthService;
//# sourceMappingURL=auth-service.js.map