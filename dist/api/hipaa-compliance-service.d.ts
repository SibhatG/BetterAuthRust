/**
 * HIPAA compliance service for handling compliance-related operations
 */
import { ApiClient } from './api-client';
import { UserRole, ResourcePermission, SessionInfo, PhiAccessLog, BaaAgreement, EmergencyAccess, RegisterBaaRequest, LogEmergencyAccessRequest, ReviewEmergencyAccessRequest, GenerateAuditReportRequest } from '../types';
export declare class HipaaComplianceService {
    private readonly apiClient;
    constructor(apiClient: ApiClient);
    /**
     * Get user role
     */
    getUserRole(): Promise<UserRole>;
    /**
     * Get permissions for current role
     */
    getRolePermissions(): Promise<ResourcePermission[]>;
    /**
     * Check permission to access a resource
     */
    checkPermission(resourceType: string, accessType: string): Promise<boolean>;
    /**
     * Get active sessions for current user
     */
    getActiveSessions(): Promise<SessionInfo[]>;
    /**
     * Terminate a session
     */
    terminateSession(sessionId: string): Promise<boolean>;
    /**
     * Get access logs
     */
    getAccessLogs(startDate: string, endDate: string): Promise<PhiAccessLog[]>;
    /**
     * Register a Business Associate Agreement
     */
    registerBaa(request: RegisterBaaRequest): Promise<BaaAgreement>;
    /**
     * Get BAA agreements
     */
    getBaaAgreements(): Promise<BaaAgreement[]>;
    /**
     * Log emergency access
     */
    logEmergencyAccess(request: LogEmergencyAccessRequest): Promise<EmergencyAccess>;
    /**
     * Review emergency access
     */
    reviewEmergencyAccess(request: ReviewEmergencyAccessRequest): Promise<boolean>;
    /**
     * Generate audit report
     */
    generateAuditReport(request: GenerateAuditReportRequest): Promise<string>;
}
