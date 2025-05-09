/**
 * Type definitions for HIPAA compliance models
 */
export declare enum UserRole {
    Patient = "Patient",
    Doctor = "Doctor",
    Nurse = "Nurse",
    Admin = "Admin",
    Technician = "Technician",
    Auditor = "Auditor"
}
export declare enum AccessType {
    View = "View",
    Create = "Create",
    Update = "Update",
    Delete = "Delete",
    Export = "Export",
    Import = "Import",
    Share = "Share",
    EmergencyAccess = "EmergencyAccess"
}
export interface ResourcePermission {
    resource_type: string;
    allowed_access_types: AccessType[];
}
export interface PhiAccessLog {
    log_id: string;
    user_id: string;
    user_name: string;
    user_role: UserRole;
    resource_id: string;
    resource_type: string;
    access_type: AccessType;
    timestamp: string;
    ip_address: string;
    user_agent: string;
    reason?: string;
}
export interface SessionInfo {
    session_id: string;
    user_id: string;
    created_at: string;
    last_activity: string;
    ip_address: string;
    user_agent: string;
    role: UserRole;
}
export interface BaaAgreement {
    entity_name: string;
    agreement_id: string;
    start_date: string;
    end_date?: string;
    signed_by: string;
    agreement_text: string;
}
export interface EmergencyAccess {
    access_id: string;
    user_id: string;
    timestamp: string;
    reason: string;
    resources_accessed: string[];
    reviewed_by?: string;
    review_timestamp?: string;
}
export interface RegisterBaaRequest {
    entity_name: string;
    agreement_text: string;
    signed_by: string;
}
export interface LogEmergencyAccessRequest {
    reason: string;
    resources: string[];
}
export interface ReviewEmergencyAccessRequest {
    access_id: string;
}
export interface GenerateAuditReportRequest {
    start_date: string;
    end_date: string;
}
