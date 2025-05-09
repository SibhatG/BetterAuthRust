/**
 * Breach detection service for handling breach detection operations
 */
import { ApiClient } from './api-client';
import { BreachCheckResult } from '../types';
export declare class BreachDetectionService {
    private readonly apiClient;
    constructor(apiClient: ApiClient);
    /**
     * Check if user's credentials have been compromised
     */
    checkBreachStatus(): Promise<BreachCheckResult>;
}
