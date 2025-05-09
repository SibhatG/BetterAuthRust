# Performance Optimizations for Better Auth

## Key Performance Bottlenecks

1. Excessive mutex locking/unlocking
2. In-memory data structures lacking proper indexing
3. Inefficient string operations
4. Lack of parallel processing for expensive operations
5. Blocking I/O operations
6. Excessive cloning of data structures

## Optimization Strategies

### 1. Mutex Optimization

**Current Implementation:**
```rust
pub fn get_user_role(&self, user_id: &Uuid) -> Option<UserRole> {
    let state = self.state.lock().unwrap();
    state.user_roles.get(user_id).cloned()
}
```

**Optimized Implementation:**
```rust
pub fn get_user_role(&self, user_id: &Uuid) -> Option<UserRole> {
    // Use try_lock for non-blocking access when possible
    if let Ok(state) = self.state.try_lock() {
        return state.user_roles.get(user_id).copied(); // Use copied instead of cloned for simple types
    }
    
    // Fall back to blocking lock if try_lock fails
    let state = self.state.lock().unwrap();
    state.user_roles.get(user_id).copied()
}
```

### 2. Read-Write Lock for Better Concurrency

**Current Implementation:**
```rust
pub struct RiskScoringContext {
    pub state: Mutex<RiskScoringState>,
}
```

**Optimized Implementation:**
```rust
use std::sync::RwLock;

pub struct RiskScoringContext {
    pub state: RwLock<RiskScoringState>,
}

// Then update methods to use read-only locks when possible
pub fn get_failed_attempts(&self, username_or_email: &str) -> u32 {
    let state = self.state.read().unwrap();
    state.failed_attempts
        .get(username_or_email)
        .map(|f| f.count)
        .unwrap_or(0)
}

pub fn record_failed_attempt(&self, username_or_email: &str) {
    let mut state = self.state.write().unwrap();
    // ...
}
```

### 3. Optimize Data Structures

**Current Implementation:**
```rust
pub struct RiskScoringState {
    pub login_history: HashMap<Uuid, Vec<LoginRecord>>,
    pub failed_attempts: HashMap<String, FailedAttempts>,
}
```

**Optimized Implementation:**
```rust
use dashmap::DashMap;  // Thread-safe concurrent HashMap

pub struct RiskScoringState {
    pub login_history: DashMap<Uuid, Vec<LoginRecord>>,
    pub failed_attempts: DashMap<String, FailedAttempts>,
}

// Update methods to use the thread-safe maps
pub fn get_failed_attempts(&self, username_or_email: &str) -> u32 {
    self.state.failed_attempts
        .get(username_or_email)
        .map(|f| f.count)
        .unwrap_or(0)
}
```

### 4. Parallel Processing for CPU-Bound Tasks

**Current Implementation:**
```rust
pub fn analyze_login_risk(&self, user_id: &Uuid, login_info: &LoginRecord) -> RiskAnalysisResult {
    let state = self.state.lock().unwrap();
    let mut risk_factors = Vec::new();
    let mut total_weight = 0;
    
    // Sequential processing of risk factors
    // ...
}
```

**Optimized Implementation:**
```rust
use rayon::prelude::*;

pub fn analyze_login_risk(&self, user_id: &Uuid, login_info: &LoginRecord) -> RiskAnalysisResult {
    let state = self.state.read().unwrap();
    
    // Get user history
    let history = match state.login_history.get(user_id) {
        Some(history) if !history.is_empty() => history.clone(),
        _ => return RiskAnalysisResult { score: 0, factors: Vec::new(), action: RiskAction::Allow }
    };
    
    // Define risk factor analyzers as closures
    let analyzers: Vec<Box<dyn Fn(&Vec<LoginRecord>, &LoginRecord) -> Option<RiskFactor> + Send>> = vec![
        Box::new(|history, login_info| check_new_device(history, login_info)),
        Box::new(|history, login_info| check_new_location(history, login_info)),
        Box::new(|history, login_info| check_impossible_travel(history, login_info)),
        Box::new(|history, login_info| check_odd_time(history, login_info)),
    ];
    
    // Run analyzers in parallel
    let risk_factors: Vec<RiskFactor> = analyzers.par_iter()
        .filter_map(|analyzer| analyzer(&history, login_info))
        .collect();
    
    // Calculate total weight
    let total_weight = risk_factors.iter().map(|f| f.weight).sum();
    
    // Determine action based on score
    let score = std::cmp::min(100, total_weight);
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
```

### 5. Async/Await for I/O-bound Operations

**Current Implementation:**
```rust
#[post("/api/auth/login")]
pub async fn login(
    req: web::Json<LoginRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Synchronous operations blocking the async function
    let users = state.users.lock().unwrap();
    // ...
}
```

**Optimized Implementation:**
```rust
#[post("/api/auth/login")]
pub async fn login(
    req: web::Json<LoginRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Move blocking operations to a separate thread
    let username_or_email = req.username_or_email.clone();
    let password = req.password.clone();
    
    // Use web::block for CPU-bound operations
    let (user, verified) = web::block(move || {
        // This runs in a thread pool to avoid blocking the async runtime
        let users = state.users.lock().unwrap();
        let user = users.values().find(|u| 
            u.username == username_or_email || u.email == username_or_email
        ).cloned();
        
        if let Some(user) = user {
            let verified = verify_password(&password, &user.password_hash).unwrap_or(false);
            (Some(user), verified)
        } else {
            (None, false)
        }
    }).await?;
    
    match user {
        Some(user) if verified => {
            // Continue with authenticated user...
        },
        _ => {
            Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                status: "error".to_string(),
                code: "INVALID_CREDENTIALS".to_string(),
                message: "Invalid username/email or password".to_string(),
            }))
        }
    }
}
```

### 6. Memory Management Optimizations

**Current Implementation:**
```rust
pub fn start_registration(
    &self,
    user_id: &Uuid,
    username: &str,
    _existing_credentials: &[WebAuthnCredential],
) -> Result<WebAuthnRegisterStartResponse, WebAuthnOperationError> {
    // Create a registration ID and challenge
    let registration_id = Uuid::new_v4().to_string();
    let challenge = Self::generate_challenge();
    
    // Create response with WebAuthn options
    let options = WebAuthnOptions {
        challenge,
        rp_id: self.rp_id.clone(),
        rp_name: self.rp_name.clone(),
        user_id: user_id.to_string(),
        username: username.to_string(),
        timeout: 60000, // 60 seconds
    };
    
    Ok(WebAuthnRegisterStartResponse {
        registration_id,
        options,
    })
}
```

**Optimized Implementation:**
```rust
pub fn start_registration<'a>(
    &'a self,
    user_id: &Uuid,
    username: &str,
    _existing_credentials: &[WebAuthnCredential],
) -> Result<WebAuthnRegisterStartResponse<'a>, WebAuthnOperationError> {
    // Create a registration ID and challenge
    let registration_id = Uuid::new_v4().to_string();
    let challenge = Self::generate_challenge();
    
    // Create response with WebAuthn options - use references instead of clones
    let options = WebAuthnOptions {
        challenge,
        rp_id: &self.rp_id,      // Use reference instead of clone
        rp_name: &self.rp_name,  // Use reference instead of clone
        user_id: user_id.to_string(),
        username: username.to_string(),
        timeout: 60000, // 60 seconds
    };
    
    Ok(WebAuthnRegisterStartResponse {
        registration_id,
        options,
    })
}

// Update struct to support references
#[derive(Debug, Serialize)]
pub struct WebAuthnOptions<'a> {
    pub challenge: String,
    pub rp_id: &'a str,
    pub rp_name: &'a str,
    pub user_id: String,
    pub username: String,
    pub timeout: u32,
}

#[derive(Debug, Serialize)]
pub struct WebAuthnRegisterStartResponse<'a> {
    pub registration_id: String,
    pub options: WebAuthnOptions<'a>,
}
```

### 7. Efficient String Handling

**Current Implementation:**
```rust
fn generate_random_email(&self) -> String {
    let random_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();
        
    format!("{}@{}", random_string.to_lowercase(), self.domain)
}
```

**Optimized Implementation:**
```rust
fn generate_random_email(&self) -> String {
    // Pre-allocate capacity for the string
    let mut random_string = String::with_capacity(12);
    
    // Generate lowercase alphanumeric directly
    for _ in 0..12 {
        let c = thread_rng().sample(Alphanumeric);
        random_string.push(char::from(c).to_ascii_lowercase());
    }
    
    // Pre-calculate capacity for the final email
    let domain_len = self.domain.len();
    let total_len = random_string.len() + 1 + domain_len; // +1 for @
    
    // Create the final email with pre-allocated capacity
    let mut email = String::with_capacity(total_len);
    email.push_str(&random_string);
    email.push('@');
    email.push_str(&self.domain);
    
    email
}
```

### 8. Batch Processing

**Current Implementation:**
```rust
pub fn check_email_breaches(&self, email: &str) -> Vec<BreachRecord> {
    let state = self.state.lock().unwrap();
    state.breached_emails
        .get(email)
        .cloned()
        .unwrap_or_default()
}
```

**Optimized Implementation:**
```rust
// For batch processing multiple emails
pub fn check_multiple_emails(&self, emails: &[String]) -> HashMap<String, Vec<BreachRecord>> {
    let state = self.state.lock().unwrap();
    emails.iter()
        .filter_map(|email| {
            state.breached_emails.get(email).map(|breaches| 
                (email.clone(), breaches.clone())
            )
        })
        .collect()
}
```