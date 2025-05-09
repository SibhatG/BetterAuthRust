use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

// Proxy email context
pub struct ProxyEmailContext {
    pub state: Mutex<ProxyEmailState>,
    pub domain: String,
}

// Proxy email state
#[derive(Debug, Clone, Default)]
pub struct ProxyEmailState {
    // Map from proxy email to real email
    pub proxy_to_real: HashMap<String, String>,
    // Map from real email to proxy emails
    pub real_to_proxies: HashMap<String, Vec<ProxyEmail>>,
    // Map of email forwarding preferences
    pub forwarding_prefs: HashMap<String, ForwardingPreferences>,
}

// Proxy email record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyEmail {
    pub proxy_address: String,
    pub real_address: String,
    pub created_at: DateTime<Utc>,
    pub label: String,
    pub status: ProxyEmailStatus,
    pub forwarding_enabled: bool,
}

// Status of a proxy email
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProxyEmailStatus {
    Active,
    Disabled,
    Deleted,
}

// Email forwarding preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardingPreferences {
    pub forward_all: bool,
    pub spam_filter_level: SpamFilterLevel,
    pub blocked_senders: Vec<String>,
    pub allowed_senders: Vec<String>,
}

// Spam filter level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpamFilterLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl Default for ForwardingPreferences {
    fn default() -> Self {
        ForwardingPreferences {
            forward_all: true,
            spam_filter_level: SpamFilterLevel::Medium,
            blocked_senders: Vec::new(),
            allowed_senders: Vec::new(),
        }
    }
}

impl ProxyEmailContext {
    pub fn new(domain: &str) -> Self {
        ProxyEmailContext {
            state: Mutex::new(ProxyEmailState::default()),
            domain: domain.to_string(),
        }
    }
    
    // Generate a random email address
    fn generate_random_email(&self) -> String {
        let random_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();
            
        format!("{}@{}", random_string.to_lowercase(), self.domain)
    }
    
    // Create a new proxy email for a user
    pub fn create_proxy_email(&self, real_email: &str, label: &str) -> ProxyEmail {
        let proxy_address = self.generate_random_email();
        
        let proxy_email = ProxyEmail {
            proxy_address: proxy_address.clone(),
            real_address: real_email.to_string(),
            created_at: Utc::now(),
            label: label.to_string(),
            status: ProxyEmailStatus::Active,
            forwarding_enabled: true,
        };
        
        let mut state = self.state.lock().unwrap();
        
        // Add mappings in both directions
        state.proxy_to_real.insert(proxy_address.clone(), real_email.to_string());
        
        let proxies = state.real_to_proxies
            .entry(real_email.to_string())
            .or_insert_with(Vec::new);
        proxies.push(proxy_email.clone());
        
        // Initialize forwarding preferences if not present
        if !state.forwarding_prefs.contains_key(real_email) {
            state.forwarding_prefs.insert(real_email.to_string(), ForwardingPreferences::default());
        }
        
        proxy_email
    }
    
    // List all proxy emails for a user
    pub fn list_proxy_emails(&self, real_email: &str) -> Vec<ProxyEmail> {
        let state = self.state.lock().unwrap();
        state.real_to_proxies
            .get(real_email)
            .cloned()
            .unwrap_or_default()
    }
    
    // Get the real email behind a proxy
    pub fn get_real_email(&self, proxy_email: &str) -> Option<String> {
        let state = self.state.lock().unwrap();
        state.proxy_to_real.get(proxy_email).cloned()
    }
    
    // Update the status of a proxy email
    pub fn update_proxy_status(&self, proxy_email: &str, status: ProxyEmailStatus) -> Option<ProxyEmail> {
        let mut state = self.state.lock().unwrap();
        
        // Find the real email first
        let real_email = match state.proxy_to_real.get(proxy_email) {
            Some(email) => email.clone(),
            None => return None,
        };
        
        // Update the status in the real_to_proxies map
        if let Some(proxies) = state.real_to_proxies.get_mut(&real_email) {
            if let Some(proxy) = proxies.iter_mut().find(|p| p.proxy_address == proxy_email) {
                proxy.status = status.clone();
                return Some(proxy.clone());
            }
        }
        
        None
    }
    
    // Delete a proxy email
    pub fn delete_proxy_email(&self, proxy_email: &str) -> bool {
        let mut state = self.state.lock().unwrap();
        
        // Find the real email first
        let real_email = match state.proxy_to_real.get(proxy_email) {
            Some(email) => email.clone(),
            None => return false,
        };
        
        // Mark as deleted in the real_to_proxies map
        if let Some(proxies) = state.real_to_proxies.get_mut(&real_email) {
            if let Some(proxy) = proxies.iter_mut().find(|p| p.proxy_address == proxy_email) {
                proxy.status = ProxyEmailStatus::Deleted;
            }
        }
        
        // Remove from the proxy_to_real map
        state.proxy_to_real.remove(proxy_email).is_some()
    }
    
    // Update forwarding preferences
    pub fn update_forwarding_preferences(&self, real_email: &str, prefs: ForwardingPreferences) {
        let mut state = self.state.lock().unwrap();
        state.forwarding_prefs.insert(real_email.to_string(), prefs);
    }
    
    // Get forwarding preferences
    pub fn get_forwarding_preferences(&self, real_email: &str) -> ForwardingPreferences {
        let state = self.state.lock().unwrap();
        state.forwarding_prefs
            .get(real_email)
            .cloned()
            .unwrap_or_default()
    }
    
    // Enable or disable forwarding for a specific proxy
    pub fn set_forwarding_enabled(&self, proxy_email: &str, enabled: bool) -> bool {
        let mut state = self.state.lock().unwrap();
        
        // Find the real email first
        let real_email = match state.proxy_to_real.get(proxy_email) {
            Some(email) => email.clone(),
            None => return false,
        };
        
        // Update forwarding status
        if let Some(proxies) = state.real_to_proxies.get_mut(&real_email) {
            if let Some(proxy) = proxies.iter_mut().find(|p| p.proxy_address == proxy_email) {
                proxy.forwarding_enabled = enabled;
                return true;
            }
        }
        
        false
    }
    
    // Simulate forwarding an email
    pub fn forward_email(&self, to: &str, from: &str, subject: &str, _body: &str) -> bool {
        let state = self.state.lock().unwrap();
        
        // Check if the proxy email exists and get the real email
        let real_email = match state.proxy_to_real.get(to) {
            Some(email) => email,
            None => return false,
        };
        
        // Get the proxy email record
        let proxy = state.real_to_proxies
            .get(real_email)
            .and_then(|proxies| proxies.iter().find(|p| p.proxy_address == to))
            .cloned();
            
        if let Some(proxy) = proxy {
            // Check if forwarding is enabled for this proxy
            if !proxy.forwarding_enabled {
                return false;
            }
            
            // Check if this sender is blocked
            let prefs = state.forwarding_prefs
                .get(real_email)
                .cloned()
                .unwrap_or_default();
                
            if prefs.blocked_senders.contains(&from.to_string()) {
                return false;
            }
            
            // In a real implementation, we would send the email here
            // For this demo, we just log it
            println!("Forwarding email: From: {} To: {} Subject: {}", from, real_email, subject);
            
            true
        } else {
            false
        }
    }
}