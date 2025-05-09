/**
 * HIPAA compliance service for handling compliance-related operations
 */

import { ApiClient } from './api-client';
import { 
  UserRole,
  ResourcePermission,
  SessionInfo,
  PhiAccessLog,
  BaaAgreement,
  EmergencyAccess,
  RegisterBaaRequest,
  LogEmergencyAccessRequest,
  ReviewEmergencyAccessRequest,
  GenerateAuditReportRequest
} from '../types';

export class HipaaComplianceService {
  private readonly apiClient: ApiClient;

  constructor(apiClient: ApiClient) {
    this.apiClient = apiClient;
  }

  /**
   * Get user role
   */
  public async getUserRole(): Promise<UserRole> {
    return this.apiClient.get<UserRole>('/api/hipaa/role');
  }

  /**
   * Get permissions for current role
   */
  public async getRolePermissions(): Promise<ResourcePermission[]> {
    return this.apiClient.get<ResourcePermission[]>('/api/hipaa/permissions');
  }

  /**
   * Check permission to access a resource
   */
  public async checkPermission(resourceType: string, accessType: string): Promise<boolean> {
    return this.apiClient.get<boolean>(
      `/api/hipaa/check-permission?resource_type=${encodeURIComponent(resourceType)}&access_type=${encodeURIComponent(accessType)}`
    );
  }

  /**
   * Get active sessions for current user
   */
  public async getActiveSessions(): Promise<SessionInfo[]> {
    return this.apiClient.get<SessionInfo[]>('/api/hipaa/sessions');
  }

  /**
   * Terminate a session
   */
  public async terminateSession(sessionId: string): Promise<boolean> {
    return this.apiClient.delete<boolean>(`/api/hipaa/sessions/${encodeURIComponent(sessionId)}`);
  }

  /**
   * Get access logs
   */
  public async getAccessLogs(startDate: string, endDate: string): Promise<PhiAccessLog[]> {
    return this.apiClient.get<PhiAccessLog[]>(
      `/api/hipaa/access-logs?start_date=${encodeURIComponent(startDate)}&end_date=${encodeURIComponent(endDate)}`
    );
  }

  /**
   * Register a Business Associate Agreement
   */
  public async registerBaa(request: RegisterBaaRequest): Promise<BaaAgreement> {
    return this.apiClient.post<BaaAgreement>('/api/hipaa/baa', request);
  }

  /**
   * Get BAA agreements
   */
  public async getBaaAgreements(): Promise<BaaAgreement[]> {
    return this.apiClient.get<BaaAgreement[]>('/api/hipaa/baa');
  }

  /**
   * Log emergency access
   */
  public async logEmergencyAccess(request: LogEmergencyAccessRequest): Promise<EmergencyAccess> {
    return this.apiClient.post<EmergencyAccess>('/api/hipaa/emergency-access', request);
  }

  /**
   * Review emergency access
   */
  public async reviewEmergencyAccess(request: ReviewEmergencyAccessRequest): Promise<boolean> {
    return this.apiClient.post<boolean>('/api/hipaa/review-emergency-access', request);
  }

  /**
   * Generate audit report
   */
  public async generateAuditReport(request: GenerateAuditReportRequest): Promise<string> {
    return this.apiClient.post<string>('/api/hipaa/audit-report', request);
  }
}