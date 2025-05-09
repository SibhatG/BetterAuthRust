"use strict";
/**
 * BetterAuth - A comprehensive authentication system client
 */
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __exportStar = (this && this.__exportStar) || function(m, exports) {
    for (var p in m) if (p !== "default" && !Object.prototype.hasOwnProperty.call(exports, p)) __createBinding(exports, m, p);
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.BetterAuthClient = void 0;
const api_1 = require("./api");
__exportStar(require("./types"), exports);
__exportStar(require("./api"), exports);
/**
 * Main client for the BetterAuth system
 */
class BetterAuthClient {
    /**
     * Creates a new BetterAuth client
     *
     * @param baseURL The base URL of the BetterAuth API
     */
    constructor(baseURL) {
        this.apiClient = new api_1.ApiClient(baseURL);
        // Initialize services
        this.auth = new api_1.AuthService(this.apiClient);
        this.riskScoring = new api_1.RiskScoringService(this.apiClient);
        this.breachDetection = new api_1.BreachDetectionService(this.apiClient);
        this.proxyEmail = new api_1.ProxyEmailService(this.apiClient);
        this.accessibility = new api_1.AccessibilityService(this.apiClient);
        this.hipaaCompliance = new api_1.HipaaComplianceService(this.apiClient);
        this.hybridEncryption = new api_1.HybridEncryptionService(this.apiClient);
    }
    /**
     * Set the authentication token manually (if needed)
     *
     * @param token The JWT token
     */
    setAuthToken(token) {
        this.apiClient.setAuthToken(token);
    }
    /**
     * Clear the authentication token
     */
    clearAuthToken() {
        this.apiClient.clearAuthToken();
    }
}
exports.BetterAuthClient = BetterAuthClient;
// Default export for easier importing
exports.default = BetterAuthClient;
//# sourceMappingURL=index.js.map