use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use chrono::{DateTime, Utc};
use uuid::Uuid;

// Breach detection context
pub struct BreachDetectionContext {
    // In-memory database of known breaches
    pub state: Mutex<BreachDetectionState>,
}

// Breach detection state
#[derive(Debug, Clone, Default)]
pub struct BreachDetectionState {
    // Map of known breached passwords (hashed)
    pub breached_passwords: HashSet<String>,
    // Map of known breached emails
    pub breached_emails: HashMap<String, Vec<BreachRecord>>,
    // Users with password reset requirements
    pub password_reset_required: HashMap<Uuid, DateTime<Utc>>,
}

// Record of a breach
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachRecord {
    pub breach_date: DateTime<Utc>,
    pub source: String,
    pub data_types: Vec<String>,
    pub description: String,
}

// Breach check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachCheckResult {
    pub is_breached: bool,
    pub breaches: Vec<BreachRecord>,
    pub password_compromised: bool,
    pub action_required: BreachAction,
}

// Actions for breach detection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BreachAction {
    None,
    PasswordReset,
    AccountLockout,
}

impl BreachDetectionContext {
    pub fn new() -> Self {
        // Initialize with some example breached passwords
        // In a real system, these would come from an API like HIBP
        let mut breached_passwords = HashSet::new();
        let common_passwords = [
            "123456", "password", "12345678", "qwerty", "123456789",
            "12345", "1234", "111111", "1234567", "dragon",
            "123123", "baseball", "abc123", "football", "monkey",
            "letmein", "shadow", "master", "666666", "qwertyuiop",
        ];
        
        for password in common_passwords.iter() {
            // In a real system, we'd use a secure hash
            breached_passwords.insert(format!("hashed_{}", password));
        }
        
        BreachDetectionContext {
            state: Mutex::new(BreachDetectionState {
                breached_passwords,
                breached_emails: HashMap::new(),
                password_reset_required: HashMap::new(),
            }),
        }
    }
    
    // Check if a password has been compromised in known breaches
    pub fn is_password_compromised(&self, password_hash: &str) -> bool {
        let state = self.state.lock().unwrap();
        state.breached_passwords.contains(password_hash)
    }
    
    // Check if an email has been involved in known breaches
    pub fn check_email_breaches(&self, email: &str) -> Vec<BreachRecord> {
        let state = self.state.lock().unwrap();
        state.breached_emails
            .get(email)
            .cloned()
            .unwrap_or_default()
    }
    
    // Add a breached password to the database
    pub fn add_breached_password(&self, password_hash: &str) {
        let mut state = self.state.lock().unwrap();
        state.breached_passwords.insert(password_hash.to_string());
    }
    
    // Add a breached email to the database
    pub fn add_breached_email(&self, email: &str, breach: BreachRecord) {
        let mut state = self.state.lock().unwrap();
        let breaches = state.breached_emails
            .entry(email.to_string())
            .or_insert_with(Vec::new);
        breaches.push(breach);
    }
    
    // Flag a user account as requiring a password reset
    pub fn require_password_reset(&self, user_id: &Uuid) {
        let mut state = self.state.lock().unwrap();
        state.password_reset_required.insert(*user_id, Utc::now());
    }
    
    // Check if a user is required to reset their password
    pub fn is_password_reset_required(&self, user_id: &Uuid) -> bool {
        let state = self.state.lock().unwrap();
        state.password_reset_required.contains_key(user_id)
    }
    
    // Clear the password reset requirement for a user
    pub fn clear_password_reset_requirement(&self, user_id: &Uuid) {
        let mut state = self.state.lock().unwrap();
        state.password_reset_required.remove(user_id);
    }
    
    // Perform a comprehensive breach check for a user
    pub fn check_user_breach(&self, email: &str, password_hash: &str, user_id: &Uuid) -> BreachCheckResult {
        let email_breaches = self.check_email_breaches(email);
        let password_compromised = self.is_password_compromised(password_hash);
        let reset_required = self.is_password_reset_required(user_id);
        
        let action = if password_compromised {
            self.require_password_reset(user_id);
            BreachAction::PasswordReset
        } else if reset_required {
            BreachAction::PasswordReset
        } else if !email_breaches.is_empty() {
            // If email was in a breach but password is ok, just monitor
            BreachAction::None
        } else {
            BreachAction::None
        };
        
        BreachCheckResult {
            is_breached: !email_breaches.is_empty() || password_compromised,
            breaches: email_breaches,
            password_compromised,
            action_required: action,
        }
    }
    
    // Simulate a Have I Been Pwned API check
    // In a real system, this would call the HIBP API
    pub fn check_hibp_api(&self, email: &str, password_hash: &str) -> BreachCheckResult {
        let email_breaches = self.check_email_breaches(email);
        let password_compromised = self.is_password_compromised(password_hash);
        
        BreachCheckResult {
            is_breached: !email_breaches.is_empty() || password_compromised,
            breaches: email_breaches,
            password_compromised,
            action_required: if password_compromised {
                BreachAction::PasswordReset
            } else {
                BreachAction::None
            },
        }
    }
}