/**
 * Type definitions for proxy email models
 */
export declare enum ProxyEmailStatus {
    Active = "Active",
    Disabled = "Disabled",
    Deleted = "Deleted"
}
export interface ProxyEmail {
    proxy_address: string;
    real_address: string;
    created_at: string;
    label: string;
    status: ProxyEmailStatus;
    forwarding_enabled: boolean;
}
export declare enum SpamFilterLevel {
    Low = "Low",
    Medium = "Medium",
    High = "High",
    VeryHigh = "VeryHigh"
}
export interface ForwardingPreferences {
    forward_all: boolean;
    spam_filter_level: SpamFilterLevel;
    blocked_senders: string[];
    allowed_senders: string[];
}
export interface CreateProxyEmailRequest {
    label: string;
}
export interface UpdateProxyEmailStatusRequest {
    proxy_address: string;
    status: ProxyEmailStatus;
    forwarding_enabled: boolean;
}
export interface ListProxyEmailsResponse {
    proxy_emails: ProxyEmail[];
}
