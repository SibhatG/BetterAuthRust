"use strict";
/**
 * HIPAA compliance service for handling compliance-related operations
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.HipaaComplianceService = void 0;
class HipaaComplianceService {
    constructor(apiClient) {
        this.apiClient = apiClient;
    }
    /**
     * Get user role
     */
    async getUserRole() {
        return this.apiClient.get('/api/hipaa/role');
    }
    /**
     * Get permissions for current role
     */
    async getRolePermissions() {
        return this.apiClient.get('/api/hipaa/permissions');
    }
    /**
     * Check permission to access a resource
     */
    async checkPermission(resourceType, accessType) {
        return this.apiClient.get(`/api/hipaa/check-permission?resource_type=${encodeURIComponent(resourceType)}&access_type=${encodeURIComponent(accessType)}`);
    }
    /**
     * Get active sessions for current user
     */
    async getActiveSessions() {
        return this.apiClient.get('/api/hipaa/sessions');
    }
    /**
     * Terminate a session
     */
    async terminateSession(sessionId) {
        return this.apiClient.delete(`/api/hipaa/sessions/${encodeURIComponent(sessionId)}`);
    }
    /**
     * Get access logs
     */
    async getAccessLogs(startDate, endDate) {
        return this.apiClient.get(`/api/hipaa/access-logs?start_date=${encodeURIComponent(startDate)}&end_date=${encodeURIComponent(endDate)}`);
    }
    /**
     * Register a Business Associate Agreement
     */
    async registerBaa(request) {
        return this.apiClient.post('/api/hipaa/baa', request);
    }
    /**
     * Get BAA agreements
     */
    async getBaaAgreements() {
        return this.apiClient.get('/api/hipaa/baa');
    }
    /**
     * Log emergency access
     */
    async logEmergencyAccess(request) {
        return this.apiClient.post('/api/hipaa/emergency-access', request);
    }
    /**
     * Review emergency access
     */
    async reviewEmergencyAccess(request) {
        return this.apiClient.post('/api/hipaa/review-emergency-access', request);
    }
    /**
     * Generate audit report
     */
    async generateAuditReport(request) {
        return this.apiClient.post('/api/hipaa/audit-report', request);
    }
}
exports.HipaaComplianceService = HipaaComplianceService;
//# sourceMappingURL=hipaa-compliance-service.js.map