/**
 * Risk scoring service for handling risk assessment operations
 */

import { ApiClient } from './api-client';
import { RiskAnalysisResponse } from '../types';

export class RiskScoringService {
  private readonly apiClient: ApiClient;

  constructor(apiClient: ApiClient) {
    this.apiClient = apiClient;
  }

  /**
   * Get risk analysis for the current user
   */
  public async getRiskAnalysis(): Promise<RiskAnalysisResponse> {
    return this.apiClient.get<RiskAnalysisResponse>('/api/risk/analysis');
  }
}