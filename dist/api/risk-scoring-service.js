"use strict";
/**
 * Risk scoring service for handling risk assessment operations
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.RiskScoringService = void 0;
class RiskScoringService {
    constructor(apiClient) {
        this.apiClient = apiClient;
    }
    /**
     * Get risk analysis for the current user
     */
    async getRiskAnalysis() {
        return this.apiClient.get('/api/risk/analysis');
    }
}
exports.RiskScoringService = RiskScoringService;
//# sourceMappingURL=risk-scoring-service.js.map