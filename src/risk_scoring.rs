use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Mutex, RwLock}; // Added RwLock for better concurrency
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration, Timelike};
use chrono::TimeZone; // Required for hour() method

// Risk factor thresholds
const RISK_THRESHOLD_BLOCK: u32 = 80;  // Block login if risk score > 80%
const RISK_THRESHOLD_MFA: u32 = 50;    // Require MFA if risk score > 50%

// Risk factors weights (out of 100)
const RISK_WEIGHT_NEW_DEVICE: u32 = 20;
const RISK_WEIGHT_NEW_LOCATION: u32 = 20;
const RISK_WEIGHT_ODD_TIME: u32 = 15;
const RISK_WEIGHT_IMPOSSIBLE_TRAVEL: u32 = 50;
const RISK_WEIGHT_MULTIPLE_FAILED_ATTEMPTS: u32 = 30;
const RISK_WEIGHT_COMPROMISED_PASSWORD: u32 = 100;

// Store for user login history
#[derive(Debug, Clone, Default)]
pub struct RiskScoringState {
    pub login_history: HashMap<Uuid, Vec<LoginRecord>>,
    pub failed_attempts: HashMap<String, FailedAttempts>,
}

// Record of a user login
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRecord {
    pub timestamp: DateTime<Utc>,
    pub ip_address: String,
    pub location: Option<GeoLocation>,
    pub device_id: String,
    pub user_agent: String,
    pub success: bool,
}

// Geographic location data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub country: String,
    pub city: String,
}

// Failed login attempts tracking
#[derive(Debug, Clone, Default)]
pub struct FailedAttempts {
    pub count: u32,
    pub first_attempt: DateTime<Utc>,
    pub last_attempt: DateTime<Utc>,
}

// Result of risk analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAnalysisResult {
    pub score: u32,  // 0-100, higher is riskier
    pub factors: Vec<RiskFactor>,
    pub action: RiskAction,
}

// Individual risk factors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub name: String,
    pub description: String,
    pub weight: u32,
}

// Actions to take based on risk score
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskAction {
    Allow,
    RequireMfa,
    Block,
}

// Risk scoring context
pub struct RiskScoringContext {
    pub state: RwLock<RiskScoringState>, // Using RwLock for better concurrency
}

// Calculate the distance between two geographic points (Haversine formula)
fn calculate_distance(loc1: &GeoLocation, loc2: &GeoLocation) -> f64 {
    const EARTH_RADIUS_KM: f64 = 6371.0;
    
    let lat1_rad = loc1.latitude.to_radians();
    let lat2_rad = loc2.latitude.to_radians();
    let delta_lat = (loc2.latitude - loc1.latitude).to_radians();
    let delta_lng = (loc2.longitude - loc1.longitude).to_radians();
    
    let a = (delta_lat / 2.0).sin().powi(2) + 
            lat1_rad.cos() * lat2_rad.cos() * (delta_lng / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    EARTH_RADIUS_KM * c
}

// Calculate the speed in km/h between two login locations
fn calculate_travel_speed(loc1: &GeoLocation, time1: DateTime<Utc>, 
                        loc2: &GeoLocation, time2: DateTime<Utc>) -> f64 {
    let distance_km = calculate_distance(loc1, loc2);
    let time_diff_hours = (time2 - time1).num_seconds() as f64 / 3600.0;
    
    if time_diff_hours <= 0.0 {
        return f64::INFINITY;  // Avoid division by zero
    }
    
    distance_km / time_diff_hours
}

// Determine if the login time is unusual for this user
fn is_unusual_time(history: &[LoginRecord], current: &DateTime<Utc>) -> bool {
    // Need at least 5 logins to establish a pattern
    if history.len() < 5 {
        return false;
    }
    
    // Check if current hour is outside the normal login hours
    let current_hour = current.hour();
    let mut hour_counts = HashMap::new();
    
    for record in history {
        if record.success {
            let hour = record.timestamp.hour();
            *hour_counts.entry(hour).or_insert(0) += 1;
        }
    }
    
    // If this hour has less than 10% of total logins, consider it unusual
    let total_logins = history.len();
    let current_hour_count = hour_counts.get(&current_hour).unwrap_or(&0);
    
    ((*current_hour_count as f64) / (total_logins as f64)) < 0.1
}

impl RiskScoringContext {
    pub fn new() -> Self {
        RiskScoringContext {
            state: RwLock::new(RiskScoringState::default()),
        }
    }
    
    // Record a login attempt
    pub fn record_login(&self, user_id: &Uuid, record: LoginRecord) {
        // Using write lock for writing operations
        let mut state = self.state.write().unwrap();
        let history = state.login_history.entry(*user_id).or_insert_with(Vec::new);
        history.push(record);
    }
    
    // Record a failed login attempt
    pub fn record_failed_attempt(&self, username_or_email: &str) {
        // Using write lock for writing operations
        let mut state = self.state.write().unwrap();
        let failed = state.failed_attempts.entry(username_or_email.to_string()).or_insert_with(|| {
            FailedAttempts {
                count: 0,
                first_attempt: Utc::now(),
                last_attempt: Utc::now(),
            }
        });
        
        failed.count += 1;
        failed.last_attempt = Utc::now();
    }
    
    // Reset failed attempts counter
    pub fn reset_failed_attempts(&self, username_or_email: &str) {
        // Using write lock for writing operations
        let mut state = self.state.write().unwrap();
        state.failed_attempts.remove(username_or_email);
    }
    
    // Get the number of failed attempts
    pub fn get_failed_attempts(&self, username_or_email: &str) -> u32 {
        // Using read lock for reading operations, allowing concurrent reads
        let state = self.state.read().unwrap();
        state.failed_attempts
            .get(username_or_email)
            .map(|f| f.count)
            .unwrap_or(0)
    }
    
    // Analyze login risk factors
    pub fn analyze_login_risk(&self, user_id: &Uuid, login_info: &LoginRecord) -> RiskAnalysisResult {
        // Using read lock for reading operations, allowing concurrent reads
        let state = self.state.read().unwrap();
        let mut risk_factors = Vec::new();
        let mut total_weight = 0;
        
        // Get user history - clone to avoid holding lock during expensive operations
        let history_clone = state.login_history.get(user_id).cloned();
        
        // Clone failed attempts for later analysis
        let failed_attempts_clone: Vec<_> = state.failed_attempts
            .values()
            .filter(|f| {
                // Check for attempts in the last hour
                (Utc::now() - f.last_attempt) < Duration::hours(1)
            })
            .cloned()
            .collect();
        
        // Drop the lock as early as possible to improve concurrency
        drop(state);
        
        if let Some(history) = history_clone {
            // Skip risk analysis if this is the first login
            if !history.is_empty() {
                // Check for new device
                let known_device = history.iter().any(|r| r.device_id == login_info.device_id);
                if !known_device {
                    risk_factors.push(RiskFactor {
                        name: "new_device".to_string(),
                        description: "Login from a new device".to_string(),
                        weight: RISK_WEIGHT_NEW_DEVICE,
                    });
                    total_weight += RISK_WEIGHT_NEW_DEVICE;
                }
                
                // Check for new location (if we have location data)
                if let Some(current_location) = &login_info.location {
                    let known_location = history.iter()
                        .filter_map(|r| r.location.as_ref())
                        .any(|loc| {
                            calculate_distance(loc, current_location) < 50.0 // Within 50km
                        });
                        
                    if !known_location {
                        risk_factors.push(RiskFactor {
                            name: "new_location".to_string(),
                            description: "Login from a new geographic location".to_string(),
                            weight: RISK_WEIGHT_NEW_LOCATION,
                        });
                        total_weight += RISK_WEIGHT_NEW_LOCATION;
                    }
                    
                    // Check for impossible travel
                    let impossible_travel = history.iter()
                        .filter(|r| r.success)
                        .filter_map(|r| r.location.as_ref().map(|loc| (loc, r.timestamp)))
                        .any(|(prev_loc, prev_time)| {
                            // Calculate travel speed between locations
                            let speed = calculate_travel_speed(
                                prev_loc, prev_time, 
                                current_location, login_info.timestamp
                            );
                            
                            // Impossible if faster than 1000 km/h (faster than commercial flight)
                            speed > 1000.0
                        });
                        
                    if impossible_travel {
                        risk_factors.push(RiskFactor {
                            name: "impossible_travel".to_string(),
                            description: "Physically impossible travel speed between login locations".to_string(),
                            weight: RISK_WEIGHT_IMPOSSIBLE_TRAVEL,
                        });
                        total_weight += RISK_WEIGHT_IMPOSSIBLE_TRAVEL;
                    }
                }
                
                // Check for unusual login time
                if is_unusual_time(&history, &login_info.timestamp) {
                    risk_factors.push(RiskFactor {
                        name: "unusual_time".to_string(),
                        description: "Login at an unusual time for this user".to_string(),
                        weight: RISK_WEIGHT_ODD_TIME,
                    });
                    total_weight += RISK_WEIGHT_ODD_TIME;
                }
            }
        }
        
        // Check for multiple failed attempts using the cloned data
        let multiple_failed = failed_attempts_clone.len() > 3;
            
        if multiple_failed {
            risk_factors.push(RiskFactor {
                name: "multiple_failed_attempts".to_string(),
                description: "Multiple failed login attempts from similar IP addresses".to_string(),
                weight: RISK_WEIGHT_MULTIPLE_FAILED_ATTEMPTS,
            });
            total_weight += RISK_WEIGHT_MULTIPLE_FAILED_ATTEMPTS;
        }
        
        // Calculate final score (0-100)
        let score = if risk_factors.is_empty() { 
            0 
        } else { 
            std::cmp::min(100, total_weight) 
        };
        
        // Determine action based on score
        let action = if score >= RISK_THRESHOLD_BLOCK {
            RiskAction::Block
        } else if score >= RISK_THRESHOLD_MFA {
            RiskAction::RequireMfa
        } else {
            RiskAction::Allow
        };
        
        RiskAnalysisResult {
            score,
            factors: risk_factors,
            action,
        }
    }
    
    // Check if a login should be blocked due to high risk
    pub fn should_block_login(&self, user_id: &Uuid, login_info: &LoginRecord) -> (bool, RiskAnalysisResult) {
        // Analyze risk (already optimized with read lock)
        let result = self.analyze_login_risk(user_id, login_info);
        (result.action == RiskAction::Block, result)
    }
    
    // Check if MFA should be required due to risk factors
    pub fn should_require_mfa(&self, user_id: &Uuid, login_info: &LoginRecord) -> (bool, RiskAnalysisResult) {
        // Analyze risk (already optimized with read lock)
        let result = self.analyze_login_risk(user_id, login_info);
        (result.action == RiskAction::RequireMfa || result.action == RiskAction::Block, result)
    }
}