use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;

// HIPAA compliance context
pub struct HipaaComplianceContext {
    pub state: Mutex<HipaaComplianceState>,
}

// HIPAA compliance state
#[derive(Default)]
pub struct HipaaComplianceState {
    // Access logs for PHI data
    pub access_logs: Vec<PhiAccessLog>,
    // Active sessions
    pub active_sessions: HashMap<String, SessionInfo>,
    // User roles and permissions
    pub user_roles: HashMap<Uuid, UserRole>,
    // BAA agreements
    pub baa_agreements: HashMap<String, BaaAgreement>,
    // Emergency access logs
    pub emergency_accesses: Vec<EmergencyAccess>,
    // Session timeouts (in seconds)
    pub session_timeouts: HashMap<UserRole, u32>,
}

// PHI access log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhiAccessLog {
    pub log_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub user_role: UserRole,
    pub resource_id: String,
    pub resource_type: String,
    pub access_type: AccessType,
    pub timestamp: DateTime<Utc>,
    pub ip_address: String,
    pub user_agent: String,
    pub reason: Option<String>,
}

// Session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub ip_address: String,
    pub user_agent: String,
    pub role: UserRole,
}

// User role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserRole {
    Patient,
    Doctor,
    Nurse,
    Admin,
    Technician,
    Auditor,
}

// Business Associate Agreement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaaAgreement {
    pub entity_name: String,
    pub agreement_id: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub signed_by: String,
    pub agreement_text: String,
}

// Emergency access information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyAccess {
    pub access_id: Uuid,
    pub user_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub reason: String,
    pub resources_accessed: Vec<String>,
    pub reviewed_by: Option<Uuid>,
    pub review_timestamp: Option<DateTime<Utc>>,
}

// Access type for PHI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccessType {
    View,
    Create,
    Update,
    Delete,
    Export,
    Import,
    Share,
    EmergencyAccess,
}

// Permission for a resource type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePermission {
    pub resource_type: String,
    pub allowed_access_types: HashSet<AccessType>,
}

impl HipaaComplianceContext {
    pub fn new() -> Self {
        let context = HipaaComplianceContext {
            state: Mutex::new(HipaaComplianceState::default()),
        };
        
        // Initialize default session timeouts per role
        context.init_session_timeouts();
        
        context
    }
    
    // Initialize default session timeouts
    fn init_session_timeouts(&self) {
        let mut state = self.state.lock().unwrap();
        
        // Set session timeouts per role (in seconds)
        state.session_timeouts.insert(UserRole::Patient, 1800);     // 30 minutes
        state.session_timeouts.insert(UserRole::Doctor, 1800);      // 30 minutes
        state.session_timeouts.insert(UserRole::Nurse, 1800);       // 30 minutes
        state.session_timeouts.insert(UserRole::Admin, 900);        // 15 minutes
        state.session_timeouts.insert(UserRole::Technician, 1200);  // 20 minutes
        state.session_timeouts.insert(UserRole::Auditor, 3600);     // 60 minutes
    }
    
    // Set a user's role
    pub fn set_user_role(&self, user_id: &Uuid, role: UserRole) {
        let mut state = self.state.lock().unwrap();
        state.user_roles.insert(*user_id, role);
    }
    
    // Get a user's role
    pub fn get_user_role(&self, user_id: &Uuid) -> Option<UserRole> {
        let state = self.state.lock().unwrap();
        state.user_roles.get(user_id).copied()
    }
    
    // Get the default permissions for a role
    pub fn get_role_permissions(&self, role: UserRole) -> Vec<ResourcePermission> {
        match role {
            UserRole::Patient => vec![
                ResourcePermission {
                    resource_type: "own_medical_records".to_string(),
                    allowed_access_types: [AccessType::View].into_iter().collect(),
                },
            ],
            UserRole::Doctor => vec![
                ResourcePermission {
                    resource_type: "patient_records".to_string(),
                    allowed_access_types: [
                        AccessType::View, 
                        AccessType::Create, 
                        AccessType::Update
                    ].into_iter().collect(),
                },
                ResourcePermission {
                    resource_type: "prescriptions".to_string(),
                    allowed_access_types: [
                        AccessType::View, 
                        AccessType::Create, 
                        AccessType::Update
                    ].into_iter().collect(),
                },
            ],
            UserRole::Nurse => vec![
                ResourcePermission {
                    resource_type: "patient_records".to_string(),
                    allowed_access_types: [
                        AccessType::View, 
                        AccessType::Update
                    ].into_iter().collect(),
                },
                ResourcePermission {
                    resource_type: "vital_signs".to_string(),
                    allowed_access_types: [
                        AccessType::View, 
                        AccessType::Create, 
                        AccessType::Update
                    ].into_iter().collect(),
                },
            ],
            UserRole::Admin => vec![
                ResourcePermission {
                    resource_type: "user_accounts".to_string(),
                    allowed_access_types: [
                        AccessType::View, 
                        AccessType::Create, 
                        AccessType::Update, 
                        AccessType::Delete
                    ].into_iter().collect(),
                },
                ResourcePermission {
                    resource_type: "system_settings".to_string(),
                    allowed_access_types: [
                        AccessType::View, 
                        AccessType::Update
                    ].into_iter().collect(),
                },
            ],
            UserRole::Technician => vec![
                ResourcePermission {
                    resource_type: "system_logs".to_string(),
                    allowed_access_types: [
                        AccessType::View
                    ].into_iter().collect(),
                },
                ResourcePermission {
                    resource_type: "system_maintenance".to_string(),
                    allowed_access_types: [
                        AccessType::View, 
                        AccessType::Update
                    ].into_iter().collect(),
                },
            ],
            UserRole::Auditor => vec![
                ResourcePermission {
                    resource_type: "access_logs".to_string(),
                    allowed_access_types: [
                        AccessType::View, 
                        AccessType::Export
                    ].into_iter().collect(),
                },
                ResourcePermission {
                    resource_type: "audit_reports".to_string(),
                    allowed_access_types: [
                        AccessType::View, 
                        AccessType::Create, 
                        AccessType::Export
                    ].into_iter().collect(),
                },
            ],
        }
    }
    
    // Check if a user has permission to access a resource
    pub fn check_permission(&self, user_id: &Uuid, resource_type: &str, access_type: AccessType) -> bool {
        let role = self.get_user_role(user_id);
        
        if let Some(role) = role {
            let permissions = self.get_role_permissions(role);
            
            for permission in permissions {
                if permission.resource_type == resource_type && 
                   permission.allowed_access_types.contains(&access_type) {
                    return true;
                }
            }
        }
        
        false
    }
    
    // Log PHI access
    pub fn log_phi_access(
        &self,
        user_id: &Uuid,
        user_name: &str,
        resource_id: &str,
        resource_type: &str,
        access_type: AccessType,
        ip_address: &str,
        user_agent: &str,
        reason: Option<String>,
    ) {
        let mut state = self.state.lock().unwrap();
        
        let role = state.user_roles.get(user_id).copied().unwrap_or(UserRole::Patient);
        
        let log = PhiAccessLog {
            log_id: Uuid::new_v4(),
            user_id: *user_id,
            user_name: user_name.to_string(),
            user_role: role,
            resource_id: resource_id.to_string(),
            resource_type: resource_type.to_string(),
            access_type,
            timestamp: Utc::now(),
            ip_address: ip_address.to_string(),
            user_agent: user_agent.to_string(),
            reason,
        };
        
        state.access_logs.push(log);
    }
    
    // Create a new session
    pub fn create_session(&self, user_id: &Uuid, session_id: &str, ip_address: &str, user_agent: &str) -> SessionInfo {
        let mut state = self.state.lock().unwrap();
        
        let role = state.user_roles.get(user_id).copied().unwrap_or(UserRole::Patient);
        
        let session = SessionInfo {
            session_id: session_id.to_string(),
            user_id: *user_id,
            created_at: Utc::now(),
            last_activity: Utc::now(),
            ip_address: ip_address.to_string(),
            user_agent: user_agent.to_string(),
            role,
        };
        
        state.active_sessions.insert(session_id.to_string(), session.clone());
        
        session
    }
    
    // Update session activity
    pub fn update_session_activity(&self, session_id: &str) -> bool {
        let mut state = self.state.lock().unwrap();
        
        if let Some(session) = state.active_sessions.get_mut(session_id) {
            session.last_activity = Utc::now();
            true
        } else {
            false
        }
    }
    
    // Check if a session is still valid
    pub fn is_session_valid(&self, session_id: &str) -> bool {
        let state = self.state.lock().unwrap();
        
        if let Some(session) = state.active_sessions.get(session_id) {
            // Get timeout for this user's role
            let timeout = state.session_timeouts
                .get(&session.role)
                .copied()
                .unwrap_or(1800); // Default to 30 minutes
                
            // Check if session has timed out
            let now = Utc::now();
            let session_age = now.signed_duration_since(session.last_activity);
            
            session_age < Duration::seconds(timeout as i64)
        } else {
            false
        }
    }
    
    // Terminate a session
    pub fn terminate_session(&self, session_id: &str) -> bool {
        let mut state = self.state.lock().unwrap();
        state.active_sessions.remove(session_id).is_some()
    }
    
    // Register a Business Associate Agreement
    pub fn register_baa(&self, entity_name: &str, agreement_text: &str, signed_by: &str) -> BaaAgreement {
        let mut state = self.state.lock().unwrap();
        
        let agreement_id = format!("BAA-{}", Uuid::new_v4());
        
        let agreement = BaaAgreement {
            entity_name: entity_name.to_string(),
            agreement_id: agreement_id.clone(),
            start_date: Utc::now(),
            end_date: None,
            signed_by: signed_by.to_string(),
            agreement_text: agreement_text.to_string(),
        };
        
        state.baa_agreements.insert(agreement_id.clone(), agreement.clone());
        
        agreement
    }
    
    // Log an emergency access event
    pub fn log_emergency_access(&self, user_id: &Uuid, reason: &str, resources: &[String]) -> EmergencyAccess {
        let mut state = self.state.lock().unwrap();
        
        let access = EmergencyAccess {
            access_id: Uuid::new_v4(),
            user_id: *user_id,
            timestamp: Utc::now(),
            reason: reason.to_string(),
            resources_accessed: resources.to_vec(),
            reviewed_by: None,
            review_timestamp: None,
        };
        
        state.emergency_accesses.push(access.clone());
        
        access
    }
    
    // Review an emergency access event
    pub fn review_emergency_access(&self, access_id: &Uuid, reviewer_id: &Uuid) -> bool {
        let mut state = self.state.lock().unwrap();
        
        for access in &mut state.emergency_accesses {
            if access.access_id == *access_id {
                access.reviewed_by = Some(*reviewer_id);
                access.review_timestamp = Some(Utc::now());
                return true;
            }
        }
        
        false
    }
    
    // Generate an audit report for a specific date range
    pub fn generate_audit_report(&self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> String {
        let state = self.state.lock().unwrap();
        
        let mut report = format!(
            "HIPAA Compliance Audit Report\n\
            ============================\n\
            Generated: {}\n\
            Period: {} to {}\n\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            start_date.format("%Y-%m-%d"),
            end_date.format("%Y-%m-%d"),
        );
        
        // Filter logs within the date range
        let logs_in_range: Vec<&PhiAccessLog> = state.access_logs.iter()
            .filter(|log| log.timestamp >= start_date && log.timestamp <= end_date)
            .collect();
            
        // Emergency accesses in the period
        let emergency_accesses: Vec<&EmergencyAccess> = state.emergency_accesses.iter()
            .filter(|access| access.timestamp >= start_date && access.timestamp <= end_date)
            .collect();
            
        // User role breakdown
        let mut role_counts = HashMap::new();
        for log in &logs_in_range {
            *role_counts.entry(log.user_role).or_insert(0) += 1;
        }
        
        // Access type breakdown
        let mut access_type_counts = HashMap::new();
        for log in &logs_in_range {
            *access_type_counts.entry(log.access_type).or_insert(0) += 1;
        }
        
        // Resource type breakdown
        let mut resource_type_counts = HashMap::new();
        for log in &logs_in_range {
            *resource_type_counts.entry(&log.resource_type).or_insert(0) += 1;
        }
        
        // Append statistics to report
        report.push_str(&format!("Total PHI accesses: {}\n", logs_in_range.len()));
        report.push_str(&format!("Emergency accesses: {}\n\n", emergency_accesses.len()));
        
        report.push_str("Access by role:\n");
        for (role, count) in role_counts {
            report.push_str(&format!("  {:?}: {}\n", role, count));
        }
        report.push_str("\n");
        
        report.push_str("Access by type:\n");
        for (access_type, count) in access_type_counts {
            report.push_str(&format!("  {:?}: {}\n", access_type, count));
        }
        report.push_str("\n");
        
        report.push_str("Access by resource type:\n");
        for (resource_type, count) in resource_type_counts {
            report.push_str(&format!("  {}: {}\n", resource_type, count));
        }
        report.push_str("\n");
        
        if !emergency_accesses.is_empty() {
            report.push_str("Emergency access details:\n");
            for access in emergency_accesses {
                report.push_str(&format!(
                    "  ID: {}\n  User ID: {}\n  Timestamp: {}\n  Reason: {}\n  Reviewed: {}\n\n",
                    access.access_id,
                    access.user_id,
                    access.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
                    access.reason,
                    match access.reviewed_by {
                        Some(_) => "Yes",
                        None => "No",
                    }
                ));
            }
        }
        
        report
    }
}