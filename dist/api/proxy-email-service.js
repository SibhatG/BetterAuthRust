"use strict";
/**
 * Proxy email service for handling "Hide My Email" operations
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.ProxyEmailService = void 0;
class ProxyEmailService {
    constructor(apiClient) {
        this.apiClient = apiClient;
    }
    /**
     * Create a new proxy email address
     */
    async createProxyEmail(request) {
        return this.apiClient.post('/api/email/create', request);
    }
    /**
     * List all proxy email addresses
     */
    async listProxyEmails() {
        return this.apiClient.get('/api/email/list');
    }
    /**
     * Update the status of a proxy email address
     */
    async updateProxyEmailStatus(request) {
        return this.apiClient.patch('/api/email/status', request);
    }
    /**
     * Delete a proxy email address
     */
    async deleteProxyEmail(proxyAddress) {
        return this.apiClient.delete(`/api/email/delete?proxy_address=${encodeURIComponent(proxyAddress)}`);
    }
    /**
     * Get forwarding preferences
     */
    async getForwardingPreferences() {
        return this.apiClient.get('/api/email/preferences');
    }
    /**
     * Update forwarding preferences
     */
    async updateForwardingPreferences(preferences) {
        return this.apiClient.patch('/api/email/preferences', preferences);
    }
}
exports.ProxyEmailService = ProxyEmailService;
//# sourceMappingURL=proxy-email-service.js.map