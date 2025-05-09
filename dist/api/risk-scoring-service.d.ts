/**
 * Risk scoring service for handling risk assessment operations
 */
import { ApiClient } from './api-client';
import { RiskAnalysisResponse } from '../types';
export declare class RiskScoringService {
    private readonly apiClient;
    constructor(apiClient: ApiClient);
    /**
     * Get risk analysis for the current user
     */
    getRiskAnalysis(): Promise<RiskAnalysisResponse>;
}
