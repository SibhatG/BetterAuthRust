"use strict";
/**
 * Hybrid encryption service for handling encryption operations
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.HybridEncryptionService = void 0;
class HybridEncryptionService {
    constructor(apiClient) {
        this.apiClient = apiClient;
    }
    /**
     * Get public keys for the current user
     */
    async getPublicKeys() {
        return this.apiClient.get('/api/encryption/keys');
    }
    /**
     * Encrypt data
     */
    async encrypt(request) {
        return this.apiClient.post('/api/encryption/encrypt', request);
    }
    /**
     * Decrypt data
     */
    async decrypt(request) {
        return this.apiClient.post('/api/encryption/decrypt', request);
    }
    /**
     * Rotate encryption keys
     */
    async rotateKeys() {
        return this.apiClient.post('/api/encryption/rotate-keys', {});
    }
    /**
     * Delete encryption keys
     */
    async deleteKeys() {
        return this.apiClient.delete('/api/encryption/keys');
    }
    /**
     * Encrypt a token
     */
    async encryptToken(token) {
        return this.apiClient.post('/api/encryption/token/encrypt', { token });
    }
    /**
     * Decrypt a token
     */
    async decryptToken(encryptedToken) {
        return this.apiClient.post('/api/encryption/token/decrypt', { encrypted_token: encryptedToken });
    }
}
exports.HybridEncryptionService = HybridEncryptionService;
//# sourceMappingURL=hybrid-encryption-service.js.map