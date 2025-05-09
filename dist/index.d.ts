/**
 * BetterAuth - A comprehensive authentication system client
 */
import { AuthService, RiskScoringService, BreachDetectionService, ProxyEmailService, AccessibilityService, HipaaComplianceService, HybridEncryptionService } from './api';
export * from './types';
export * from './api';
/**
 * Main client for the BetterAuth system
 */
export declare class BetterAuthClient {
    private readonly apiClient;
    readonly auth: AuthService;
    readonly riskScoring: RiskScoringService;
    readonly breachDetection: BreachDetectionService;
    readonly proxyEmail: ProxyEmailService;
    readonly accessibility: AccessibilityService;
    readonly hipaaCompliance: HipaaComplianceService;
    readonly hybridEncryption: HybridEncryptionService;
    /**
     * Creates a new BetterAuth client
     *
     * @param baseURL The base URL of the BetterAuth API
     */
    constructor(baseURL: string);
    /**
     * Set the authentication token manually (if needed)
     *
     * @param token The JWT token
     */
    setAuthToken(token: string): void;
    /**
     * Clear the authentication token
     */
    clearAuthToken(): void;
}
export default BetterAuthClient;
