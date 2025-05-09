/**
 * Proxy email service for handling "Hide My Email" operations
 */
import { ApiClient } from './api-client';
import { CreateProxyEmailRequest, ProxyEmail, UpdateProxyEmailStatusRequest, ListProxyEmailsResponse, ForwardingPreferences } from '../types';
export declare class ProxyEmailService {
    private readonly apiClient;
    constructor(apiClient: ApiClient);
    /**
     * Create a new proxy email address
     */
    createProxyEmail(request: CreateProxyEmailRequest): Promise<ProxyEmail>;
    /**
     * List all proxy email addresses
     */
    listProxyEmails(): Promise<ListProxyEmailsResponse>;
    /**
     * Update the status of a proxy email address
     */
    updateProxyEmailStatus(request: UpdateProxyEmailStatusRequest): Promise<ProxyEmail>;
    /**
     * Delete a proxy email address
     */
    deleteProxyEmail(proxyAddress: string): Promise<void>;
    /**
     * Get forwarding preferences
     */
    getForwardingPreferences(): Promise<ForwardingPreferences>;
    /**
     * Update forwarding preferences
     */
    updateForwardingPreferences(preferences: ForwardingPreferences): Promise<ForwardingPreferences>;
}
