"use strict";
/**
 * Breach detection service for handling breach detection operations
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.BreachDetectionService = void 0;
class BreachDetectionService {
    constructor(apiClient) {
        this.apiClient = apiClient;
    }
    /**
     * Check if user's credentials have been compromised
     */
    async checkBreachStatus() {
        return this.apiClient.get('/api/security/breach-check');
    }
}
exports.BreachDetectionService = BreachDetectionService;
//# sourceMappingURL=breach-detection-service.js.map