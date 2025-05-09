/**
 * Breach detection service for handling breach detection operations
 */

import { ApiClient } from './api-client';
import { BreachCheckResult } from '../types';

export class BreachDetectionService {
  private readonly apiClient: ApiClient;

  constructor(apiClient: ApiClient) {
    this.apiClient = apiClient;
  }

  /**
   * Check if user's credentials have been compromised
   */
  public async checkBreachStatus(): Promise<BreachCheckResult> {
    return this.apiClient.get<BreachCheckResult>('/api/security/breach-check');
  }
}