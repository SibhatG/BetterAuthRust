/**
 * Proxy email service for handling "Hide My Email" operations
 */

import { ApiClient } from './api-client';
import { 
  CreateProxyEmailRequest, 
  ProxyEmail, 
  UpdateProxyEmailStatusRequest,
  ListProxyEmailsResponse,
  ForwardingPreferences
} from '../types';

export class ProxyEmailService {
  private readonly apiClient: ApiClient;

  constructor(apiClient: ApiClient) {
    this.apiClient = apiClient;
  }

  /**
   * Create a new proxy email address
   */
  public async createProxyEmail(request: CreateProxyEmailRequest): Promise<ProxyEmail> {
    return this.apiClient.post<ProxyEmail>('/api/email/create', request);
  }

  /**
   * List all proxy email addresses
   */
  public async listProxyEmails(): Promise<ListProxyEmailsResponse> {
    return this.apiClient.get<ListProxyEmailsResponse>('/api/email/list');
  }

  /**
   * Update the status of a proxy email address
   */
  public async updateProxyEmailStatus(request: UpdateProxyEmailStatusRequest): Promise<ProxyEmail> {
    return this.apiClient.patch<ProxyEmail>('/api/email/status', request);
  }

  /**
   * Delete a proxy email address
   */
  public async deleteProxyEmail(proxyAddress: string): Promise<void> {
    return this.apiClient.delete<void>(`/api/email/delete?proxy_address=${encodeURIComponent(proxyAddress)}`);
  }

  /**
   * Get forwarding preferences
   */
  public async getForwardingPreferences(): Promise<ForwardingPreferences> {
    return this.apiClient.get<ForwardingPreferences>('/api/email/preferences');
  }

  /**
   * Update forwarding preferences
   */
  public async updateForwardingPreferences(preferences: ForwardingPreferences): Promise<ForwardingPreferences> {
    return this.apiClient.patch<ForwardingPreferences>('/api/email/preferences', preferences);
  }
}