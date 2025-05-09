/**
 * BetterAuth - A comprehensive authentication system client
 */

import { 
  ApiClient,
  AuthService,
  RiskScoringService,
  BreachDetectionService,
  ProxyEmailService,
  AccessibilityService,
  HipaaComplianceService,
  HybridEncryptionService
} from './api';

export * from './types';
export * from './api';

/**
 * Main client for the BetterAuth system
 */
export class BetterAuthClient {
  private readonly apiClient: ApiClient;
  
  // Services
  public readonly auth: AuthService;
  public readonly riskScoring: RiskScoringService;
  public readonly breachDetection: BreachDetectionService;
  public readonly proxyEmail: ProxyEmailService;
  public readonly accessibility: AccessibilityService;
  public readonly hipaaCompliance: HipaaComplianceService;
  public readonly hybridEncryption: HybridEncryptionService;

  /**
   * Creates a new BetterAuth client
   * 
   * @param baseURL The base URL of the BetterAuth API
   */
  constructor(baseURL: string) {
    this.apiClient = new ApiClient(baseURL);
    
    // Initialize services
    this.auth = new AuthService(this.apiClient);
    this.riskScoring = new RiskScoringService(this.apiClient);
    this.breachDetection = new BreachDetectionService(this.apiClient);
    this.proxyEmail = new ProxyEmailService(this.apiClient);
    this.accessibility = new AccessibilityService(this.apiClient);
    this.hipaaCompliance = new HipaaComplianceService(this.apiClient);
    this.hybridEncryption = new HybridEncryptionService(this.apiClient);
  }

  /**
   * Set the authentication token manually (if needed)
   * 
   * @param token The JWT token
   */
  public setAuthToken(token: string): void {
    this.apiClient.setAuthToken(token);
  }

  /**
   * Clear the authentication token
   */
  public clearAuthToken(): void {
    this.apiClient.clearAuthToken();
  }
}

// Default export for easier importing
export default BetterAuthClient;