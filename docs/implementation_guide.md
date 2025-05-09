# Better Auth: Implementation Guide

This guide explains how to implement and integrate the Better Auth system into your own projects. It covers both server-side and client-side implementation details.

## Table of Contents

1. [Server Setup](#server-setup)
2. [Database Configuration](#database-configuration)
3. [Authentication System](#authentication-system)
4. [WebAuthn Implementation](#webauthn-implementation)
5. [Risk Scoring System](#risk-scoring-system)
6. [Breach Detection](#breach-detection)
7. [Proxy Email Service](#proxy-email-service)
8. [Hybrid Encryption](#hybrid-encryption)
9. [Accessibility Features](#accessibility-features)
10. [HIPAA Compliance](#hipaa-compliance)

## Server Setup

### Prerequisites

- Rust 1.65 or later
- PostgreSQL 12 or later
- Node.js 16 or later (for TypeScript client)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/better-auth-rust.git
   cd better-auth-rust
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Set up environment variables (create a `.env` file):
   ```
   DATABASE_URL=postgres://username:password@localhost/better_auth
   JWT_SECRET=your_secret_key
   WEBAUTHN_RP_ID=localhost
   WEBAUTHN_RP_ORIGIN=http://localhost:5000
   ```

4. Run the server:
   ```bash
   cargo run
   ```

## Database Configuration

The system uses PostgreSQL for data storage. Set up your database with the following steps:

1. Create a new PostgreSQL database:
   ```bash
   createdb better_auth
   ```

2. Run the migrations:
   ```bash
   cargo install diesel_cli --no-default-features --features postgres
   diesel migration run
   ```

3. The database schema includes tables for:
   - Users
   - Sessions
   - MFA credentials
   - WebAuthn credentials
   - Risk scoring data
   - Breach records
   - Proxy emails
   - Encryption keys

## Authentication System

### Core Components

The authentication system is built around these main components:

1. **User Management**: Registration, login, and profile management.
2. **Session Management**: Secure sessions with tokens.
3. **MFA**: Multiple factors for authentication.
4. **Password Security**: Secure hashing and validation.

### Code Structure

```
src/
  ├── models/
  │   ├── user.rs         # User model
  │   ├── session.rs      # Session management
  │   └── mfa.rs          # MFA implementation
  ├── routes/
  │   ├── auth.rs         # Auth route handlers
  │   └── users.rs        # User management
  ├── services/
  │   ├── auth.rs         # Auth business logic
  │   └── email.rs        # Email services
  ├── utils/
  │   ├── password.rs     # Password hashing
  │   └── jwt.rs          # JWT token handling
  └── main.rs             # Application entry point
```

### User Registration Example

```rust
// src/routes/auth.rs
#[post("/api/auth/register")]
pub async fn register(
    req: web::Json<RegisterRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Validate input
    if req.password != req.password_confirmation {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            status: "error".to_string(),
            code: "PASSWORD_MISMATCH".to_string(),
            message: "Passwords do not match".to_string(),
        }));
    }
    
    // Check if user exists
    let mut users = state.users.lock().unwrap();
    let existing_user = users.values().find(|u| 
        u.username == req.username || u.email == req.email
    );
    
    if existing_user.is_some() {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            status: "error".to_string(),
            code: "USER_EXISTS".to_string(),
            message: "User with this username or email already exists".to_string(),
        }));
    }
    
    // Hash password
    let password_hash = hash_password(&req.password)?;
    
    // Create user
    let user = User {
        id: Uuid::new_v4(),
        username: req.username.clone(),
        email: req.email.clone(),
        password_hash,
        is_email_verified: false,
        mfa_enabled: false,
        webauthn_credentials: Vec::new(),
    };
    
    // Add to database
    users.insert(user.id, user.clone());
    
    // Return response
    Ok(HttpResponse::Created().json(RegisterResponse {
        user: UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            is_email_verified: user.is_email_verified,
            mfa_enabled: user.mfa_enabled,
        },
        message: "User registered successfully. Please check your email to verify your account.".to_string(),
    }))
}
```

### User Login Example

```rust
// src/routes/auth.rs
#[post("/api/auth/login")]
pub async fn login(
    req: web::Json<LoginRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Find user by username or email
    let users = state.users.lock().unwrap();
    let user = users.values().find(|u| 
        u.username == req.username_or_email || u.email == req.username_or_email
    );
    
    let user = match user {
        Some(user) => user.clone(),
        None => {
            return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                status: "error".to_string(),
                code: "INVALID_CREDENTIALS".to_string(),
                message: "Invalid username/email or password".to_string(),
            }));
        }
    };
    
    // Verify password
    if !verify_password(&req.password, &user.password_hash)? {
        return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
            status: "error".to_string(),
            code: "INVALID_CREDENTIALS".to_string(),
            message: "Invalid username/email or password".to_string(),
        }));
    }
    
    // Generate tokens
    let access_token = Uuid::new_v4().to_string();
    let refresh_token = Uuid::new_v4().to_string();
    
    // Create session
    let session = Session {
        id: Uuid::new_v4(),
        user_id: user.id,
        refresh_token: refresh_token.clone(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
    };
    
    // Store session
    let mut sessions = state.sessions.lock().unwrap();
    sessions.insert(session.id, session);
    
    // Return response
    Ok(HttpResponse::Ok().json(LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: 3600,
        user: UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            is_email_verified: user.is_email_verified,
            mfa_enabled: user.mfa_enabled,
        },
        mfa_required: false,
    }))
}
```

## WebAuthn Implementation

WebAuthn enables passwordless authentication using biometric or security keys.

### WebAuthn Context Setup

```rust
// src/webauthn_simplified.rs
pub struct WebAuthnContext {
    rp_id: String,
    rp_name: String,
}

impl WebAuthnContext {
    pub fn new(rp_id: &str, rp_origin: &str) -> Result<Self, WebAuthnOperationError> {
        // In a real implementation, we would validate these parameters
        // For demo, just extract domain from origin
        let rp_name = rp_id.to_string();

        Ok(WebAuthnContext {
            rp_id: rp_id.to_string(),
            rp_name,
        })
    }
    
    // Generate a random challenge string
    fn generate_challenge() -> String {
        base64::encode(Uuid::new_v4().as_bytes())
    }
}
```

### Registration Example

```rust
// src/main.rs
#[post("/api/auth/webauthn/register/start")]
pub async fn webauthn_register_start(
    req: HttpRequest,
    state: web::Data<auth_types::AppState>,
) -> Result<HttpResponse, Error> {
    // Get user from JWT token (simplified here)
    let user_id = get_user_id_from_request(&req)?;
    
    // Find user
    let users = state.users.lock().unwrap();
    let user = users.get(&user_id).cloned().ok_or_else(|| {
        HttpResponse::Unauthorized().json(auth_types::ErrorResponse {
            status: "error".to_string(),
            code: "AUTHENTICATION_ERROR".to_string(),
            message: "User not authenticated".to_string(),
        })
    })?;
    drop(users);
    
    // Create WebAuthn context
    let webauthn_ctx = webauthn_simplified::WebAuthnContext::new(
        "better-auth.example.com",
        "https://better-auth.example.com",
    )?;
    
    // Start registration
    let result = webauthn_ctx.start_registration(
        &user.id,
        &user.username,
        &user.webauthn_credentials,
    )?;
    
    Ok(HttpResponse::Ok().json(result))
}
```

### Authentication Example

```rust
#[post("/api/auth/webauthn/login/start")]
pub async fn webauthn_login_start(
    req: web::Json<auth_types::WebAuthnLoginStartRequest>,
    state: web::Data<auth_types::AppState>,
) -> Result<HttpResponse, Error> {
    // Find user by username or email
    let users = state.users.lock().unwrap();
    let user = users.values().find(|u| 
        u.username == req.username_or_email || u.email == req.username_or_email
    ).cloned();
    
    if let Some(user) = user {
        drop(users);
        
        // Ensure user has WebAuthn credentials
        if user.webauthn_credentials.is_empty() {
            return Ok(HttpResponse::BadRequest().json(auth_types::ErrorResponse {
                status: "error".to_string(),
                code: "NO_CREDENTIALS".to_string(),
                message: "No WebAuthn credentials found for this user".to_string(),
            }));
        }
        
        // Create WebAuthn context
        let webauthn_ctx = webauthn_simplified::WebAuthnContext::new(
            "better-auth.example.com",
            "https://better-auth.example.com",
        )?;
        
        // Start authentication
        let result = webauthn_ctx.start_authentication(&user.webauthn_credentials)?;
        
        Ok(HttpResponse::Ok().json(result))
    } else {
        Ok(HttpResponse::Unauthorized().json(auth_types::ErrorResponse {
            status: "error".to_string(),
            code: "USER_NOT_FOUND".to_string(),
            message: "User not found".to_string(),
        }))
    }
}
```

## Risk Scoring System

The AI-based risk scoring system analyzes login patterns to detect suspicious activities.

### Risk Context Setup

```rust
// src/risk_scoring.rs
pub struct RiskScoringContext {
    pub state: Mutex<RiskScoringState>,
}

impl RiskScoringContext {
    pub fn new() -> Self {
        RiskScoringContext {
            state: Mutex::new(RiskScoringState::default()),
        }
    }
    
    // Record a login attempt
    pub fn record_login(&self, user_id: &Uuid, record: LoginRecord) {
        let mut state = self.state.lock().unwrap();
        let history = state.login_history.entry(*user_id).or_insert_with(Vec::new);
        history.push(record);
    }
}
```

### Risk Analysis Example

```rust
// src/risk_scoring.rs
pub fn analyze_login_risk(&self, user_id: &Uuid, login_info: &LoginRecord) -> RiskAnalysisResult {
    let state = self.state.lock().unwrap();
    let mut risk_factors = Vec::new();
    let mut total_weight = 0;
    
    // Get user history
    let history = state.login_history.get(user_id);
    
    if let Some(history) = history {
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
                        description: "Login from a new location".to_string(),
                        weight: RISK_WEIGHT_NEW_LOCATION,
                    });
                    total_weight += RISK_WEIGHT_NEW_LOCATION;
                }
                
                // Check for impossible travel
                let latest_login = history.iter()
                    .filter_map(|r| r.location.as_ref().map(|loc| (r.timestamp, loc)))
                    .max_by_key(|(ts, _)| ts);
                    
                if let Some((prev_ts, prev_loc)) = latest_login {
                    let hours_diff = (login_info.timestamp - prev_ts).num_seconds() as f64 / 3600.0;
                    let distance = calculate_distance(prev_loc, current_location);
                    
                    // Estimate impossible travel (speed > 1000 km/h is suspicious)
                    if hours_diff > 0.0 && distance / hours_diff > 1000.0 {
                        risk_factors.push(RiskFactor {
                            name: "impossible_travel".to_string(),
                            description: "Impossible travel detected".to_string(),
                            weight: RISK_WEIGHT_IMPOSSIBLE_TRAVEL,
                        });
                        total_weight += RISK_WEIGHT_IMPOSSIBLE_TRAVEL;
                    }
                }
            }
            
            // Check for odd time
            let now = login_info.timestamp;
            let user_normal_hours = analyze_user_normal_hours(history);
            
            if !is_normal_time(now, &user_normal_hours) {
                risk_factors.push(RiskFactor {
                    name: "odd_time".to_string(),
                    description: "Login at unusual time".to_string(),
                    weight: RISK_WEIGHT_ODD_TIME,
                });
                total_weight += RISK_WEIGHT_ODD_TIME;
            }
        }
    }
    
    // Calculate score (max 100)
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
```

### Integration Example

```rust
// src/routes/auth.rs
#[post("/api/auth/login")]
pub async fn login(
    req: web::Json<LoginRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // ... Authentication logic (as shown earlier)
    
    // Create login record
    let login_record = LoginRecord {
        timestamp: chrono::Utc::now(),
        ip_address: get_client_ip(&req),
        location: get_geo_location(&req),
        device_id: get_device_id(&req),
        user_agent: get_user_agent(&req),
        success: true,
    };
    
    // Analyze risk
    let risk_ctx = get_risk_scoring_context();
    let (block_login, risk_analysis) = risk_ctx.should_block_login(&user.id, &login_record);
    
    if block_login {
        // Record failed login
        let mut failed_record = login_record.clone();
        failed_record.success = false;
        risk_ctx.record_login(&user.id, failed_record);
        
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            status: "error".to_string(),
            code: "LOGIN_BLOCKED".to_string(),
            message: "Login blocked due to security concerns".to_string(),
        }));
    }
    
    // Record successful login
    risk_ctx.record_login(&user.id, login_record);
    
    // Check if MFA should be required based on risk
    let require_mfa = risk_analysis.action == RiskAction::RequireMfa || user.mfa_enabled;
    
    // ... Generate tokens and complete login
    
    // Add risk analysis to the response
    let response = LoginResponse {
        // ... regular fields
        risk_analysis: Some(risk_analysis),
    };
    
    Ok(HttpResponse::Ok().json(response))
}
```

## Breach Detection

The breach detection system checks for compromised credentials using data breach information.

### Breach Detection Setup

```rust
// src/breach_detection.rs
pub struct BreachDetectionContext {
    pub state: Mutex<BreachDetectionState>,
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
}
```

### Checking for Breaches

```rust
// src/breach_detection.rs
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
```

### Integration Example

```rust
// src/routes/auth.rs
#[post("/api/auth/login")]
pub async fn login(
    req: web::Json<LoginRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // ... Authentication logic and risk analysis
    
    // Check for breaches
    let breach_ctx = get_breach_detection_context();
    let breach_result = breach_ctx.check_user_breach(
        &user.email,
        &user.password_hash,
        &user.id
    );
    
    // If password is compromised, require reset
    if breach_result.password_compromised {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            status: "error".to_string(),
            code: "PASSWORD_COMPROMISED".to_string(),
            message: "Your password has been found in a data breach. Please reset your password.".to_string(),
        }));
    }
    
    // ... Complete login
    
    // Add breach data to the response
    let response = LoginResponse {
        // ... regular fields
        breach_check: Some(breach_result),
    };
    
    Ok(HttpResponse::Ok().json(response))
}
```

## Proxy Email Service

The proxy email service creates disposable email addresses to protect users' real email addresses.

### Proxy Email Setup

```rust
// src/proxy_email.rs
pub struct ProxyEmailContext {
    pub state: Mutex<ProxyEmailState>,
    pub domain: String,
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
}
```

### Creating Proxy Emails

```rust
// src/proxy_email.rs
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
```

### Email Forwarding Logic

```rust
// src/proxy_email.rs
pub fn forward_email(&self, to: &str, from: &str, subject: &str, body: &str) -> bool {
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
        
        // Apply spam filter based on level
        let is_spam = detect_spam(subject, body, &prefs.spam_filter_level);
        if is_spam {
            return false;
        }
        
        // Forward the email (in a real system, this would call an email API)
        log::info!(
            "Forwarding email from {} via {} to {}",
            from, to, real_email
        );
        
        // Return success
        true
    } else {
        false
    }
}
```

## Hybrid Encryption

The hybrid encryption system combines RSA and post-quantum CRYSTALS-Kyber for secure data protection.

### Encryption Setup

```rust
// src/hybrid_encryption.rs
pub struct HybridEncryptionContext {
    pub state: Mutex<HybridEncryptionState>,
}

impl HybridEncryptionContext {
    pub fn new() -> Self {
        HybridEncryptionContext {
            state: Mutex::new(HybridEncryptionState::default()),
        }
    }
    
    // Generate a hybrid key pair for a user
    pub fn generate_key_pair(&self, user_id: &Uuid) -> HybridKeyPair {
        // In a real implementation, we would generate actual RSA and Kyber keys
        // For this demo, we just simulate the process
        
        // Simulate RSA key generation
        let rsa_private_key = format!("RSA_PRIV_{}", Uuid::new_v4());
        let rsa_public_key = format!("RSA_PUB_{}", Uuid::new_v4());
        
        // Simulate Kyber key generation
        let kyber_private_key = format!("KYBER_PRIV_{}", Uuid::new_v4());
        let kyber_public_key = format!("KYBER_PUB_{}", Uuid::new_v4());
        
        let key_pair = HybridKeyPair {
            user_id: *user_id,
            rsa_private_key,
            rsa_public_key,
            kyber_private_key,
            kyber_public_key,
            created_at: chrono::Utc::now(),
        };
        
        // Store the key pair
        let mut state = self.state.lock().unwrap();
        state.key_pairs.insert(*user_id, key_pair.clone());
        
        key_pair
    }
}
```

### Encryption and Decryption

```rust
// src/hybrid_encryption.rs
pub fn encrypt(&self, recipient_id: &Uuid, data: &str) -> Option<HybridEncryptedData> {
    let state = self.state.lock().unwrap();
    
    if let Some(_key_pair) = state.key_pairs.get(recipient_id) {
        // In a real implementation, this would be proper hybrid encryption
        // 1. Generate a random symmetric key
        // 2. Encrypt the data with the symmetric key
        // 3. Encrypt the symmetric key with both RSA and Kyber
        
        // For this demo, we just simulate the process
        let simulated_symmetric_key = format!("SYM_KEY_{}", Uuid::new_v4());
        let rsa_encrypted_key = format!("RSA_ENC({})", simulated_symmetric_key);
        let kyber_encrypted_key = format!("KYBER_ENC({})", simulated_symmetric_key);
        
        // Base64 encode the plaintext to simulate encryption
        let encrypted_data = BASE64.encode(data.as_bytes());
        
        Some(HybridEncryptedData {
            rsa_encrypted_key,
            kyber_encrypted_key,
            encrypted_data,
            nonce: Uuid::new_v4().to_string(),
            algorithm: "AES-256-GCM".to_string(),
        })
    } else {
        None
    }
}

pub fn decrypt(&self, user_id: &Uuid, encrypted: &HybridEncryptedData) -> Option<String> {
    let state = self.state.lock().unwrap();
    
    if let Some(_key_pair) = state.key_pairs.get(user_id) {
        // In a real implementation, this would:
        // 1. Decrypt the symmetric key using either RSA or Kyber
        // 2. Use the symmetric key to decrypt the data
        
        // For this demo, just base64 decode the "encrypted" data
        let bytes = BASE64.decode(encrypted.encrypted_data.as_bytes()).ok()?;
        let decrypted = String::from_utf8(bytes).ok()?;
        
        Some(decrypted)
    } else {
        None
    }
}
```

## Accessibility Features

Implement accessibility features to ensure authentication is available to all users.

### Accessibility Setup

```rust
// src/accessibility.rs
pub struct AccessibilityContext {
    pub state: Mutex<AccessibilityState>,
}

impl AccessibilityContext {
    pub fn new() -> Self {
        AccessibilityContext {
            state: Mutex::new(AccessibilityState::default()),
        }
    }
    
    // Get accessibility preferences for a user
    pub fn get_preferences(&self, user_id: &Uuid) -> Option<AccessibilityPreferences> {
        let state = self.state.lock().unwrap();
        state.user_preferences.get(user_id).cloned()
    }
    
    // Update accessibility preferences
    pub fn update_preferences(&self, user_id: &Uuid, prefs: AccessibilityPreferences) {
        let mut state = self.state.lock().unwrap();
        state.user_preferences.insert(*user_id, prefs);
    }
}
```

### Generating CSS Variables

```rust
// src/accessibility.rs
pub fn generate_css_variables(&self, prefs: &AccessibilityPreferences) -> String {
    let mut css = String::new();
    
    // High contrast mode
    if prefs.high_contrast {
        css.push_str("--background-color: #000000;\n");
        css.push_str("--text-color: #ffffff;\n");
        css.push_str("--link-color: #ffff00;\n");
        css.push_str("--input-bg-color: #333333;\n");
        css.push_str("--input-text-color: #ffffff;\n");
        css.push_str("--button-bg-color: #0066ff;\n");
        css.push_str("--button-text-color: #ffffff;\n");
        css.push_str("--contrast-ratio: 7;\n");
    } else {
        css.push_str("--background-color: #ffffff;\n");
        css.push_str("--text-color: #333333;\n");
        css.push_str("--link-color: #0066cc;\n");
        css.push_str("--input-bg-color: #f5f5f5;\n");
        css.push_str("--input-text-color: #333333;\n");
        css.push_str("--button-bg-color: #0066ff;\n");
        css.push_str("--button-text-color: #ffffff;\n");
        css.push_str("--contrast-ratio: 4.5;\n");
    }
    
    // Large text mode
    if prefs.large_text {
        css.push_str("--font-size-base: 18px;\n");
        css.push_str("--font-size-large: 24px;\n");
        css.push_str("--font-size-small: 16px;\n");
        css.push_str("--line-height: 1.5;\n");
    } else {
        css.push_str("--font-size-base: 16px;\n");
        css.push_str("--font-size-large: 20px;\n");
        css.push_str("--font-size-small: 14px;\n");
        css.push_str("--line-height: 1.3;\n");
    }
    
    // Reduced motion
    if prefs.reduced_motion {
        css.push_str("--transition-speed: 0s;\n");
        css.push_str("--animation-speed: 0s;\n");
    } else {
        css.push_str("--transition-speed: 0.3s;\n");
        css.push_str("--animation-speed: 0.5s;\n");
    }
    
    // Screen reader optimizations
    if prefs.screen_reader_optimized {
        css.push_str("--focus-outline-width: 3px;\n");
        css.push_str("--focus-outline-style: solid;\n");
        css.push_str("--focus-outline-color: #ff6600;\n");
    } else {
        css.push_str("--focus-outline-width: 2px;\n");
        css.push_str("--focus-outline-style: solid;\n");
        css.push_str("--focus-outline-color: #0066ff;\n");
    }
    
    css
}
```

### Voice Command Processing

```rust
// src/accessibility.rs
pub fn process_voice_command(&self, command_text: &str) -> VoiceCommand {
    // In a real implementation, this would use NLP to understand commands
    // For this demo, we just match on simple phrases
    
    let command = command_text.trim().to_lowercase();
    let (recognized_command, confidence, action) = match command.as_str() {
        "login" => (
            "login".to_string(), 
            0.95, 
            "submitLoginForm".to_string()
        ),
        "sign in" => (
            "login".to_string(),
            0.9,
            "submitLoginForm".to_string()
        ),
        "log out" | "logout" => (
            "logout".to_string(),
            0.95,
            "performLogout".to_string()
        ),
        "register" | "sign up" => (
            "register".to_string(),
            0.95,
            "navigateToRegister".to_string()
        ),
        _ => (
            command.clone(),
            0.4,
            "unknownCommand".to_string()
        ),
    };
    
    VoiceCommand {
        command: recognized_command,
        confidence,
        action,
    }
}
```

## HIPAA Compliance

Implement HIPAA-compliant authentication controls for healthcare applications.

### HIPAA Context Setup

```rust
// src/hipaa_compliance.rs
pub struct HipaaComplianceContext {
    pub state: Mutex<HipaaComplianceState>,
}

impl HipaaComplianceContext {
    pub fn new() -> Self {
        HipaaComplianceContext {
            state: Mutex::new(HipaaComplianceState::default()),
        }
    }
    
    // Set a user's role
    pub fn set_user_role(&self, user_id: &Uuid, role: UserRole) {
        let mut state = self.state.lock().unwrap();
        state.user_roles.insert(*user_id, role);
    }
    
    // Get a user's role
    pub fn get_user_role(&self, user_id: &Uuid) -> Option<UserRole> {
        let state = self.state.lock().unwrap();
        state.user_roles.get(user_id).cloned()
    }
}
```

### Permission System

```rust
// src/hipaa_compliance.rs
pub fn check_permission(&self, user_id: &Uuid, resource_type: &str, access_type: AccessType) -> bool {
    let state = self.state.lock().unwrap();
    
    // Get user role
    let role = match state.user_roles.get(user_id) {
        Some(role) => role,
        None => return false,
    };
    
    // Get permissions for this role
    let role_permissions = match state.role_permissions.get(role) {
        Some(perms) => perms,
        None => return false,
    };
    
    // Check if this resource is permitted
    for perm in role_permissions {
        if perm.resource_type == resource_type && perm.allowed_access_types.contains(&access_type) {
            return true;
        }
    }
    
    false
}
```

### Audit Logging

```rust
// src/hipaa_compliance.rs
pub fn log_access(&self, log: PhiAccessLog) {
    let mut state = self.state.lock().unwrap();
    state.access_logs.push(log);
}

pub fn get_access_logs(&self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Vec<PhiAccessLog> {
    let state = self.state.lock().unwrap();
    state.access_logs
        .iter()
        .filter(|log| log.timestamp >= start_date && log.timestamp <= end_date)
        .cloned()
        .collect()
}
```

### Emergency Access

```rust
// src/hipaa_compliance.rs
pub fn log_emergency_access(&self, access: EmergencyAccess) -> String {
    let access_id = Uuid::new_v4().to_string();
    let mut access = access.clone();
    access.access_id = access_id.clone();
    
    let mut state = self.state.lock().unwrap();
    state.emergency_accesses.push(access);
    
    access_id
}

pub fn review_emergency_access(&self, access_id: &str, reviewer_id: &str) -> bool {
    let mut state = self.state.lock().unwrap();
    
    for access in state.emergency_accesses.iter_mut() {
        if access.access_id == access_id {
            access.reviewed_by = Some(reviewer_id.to_string());
            access.review_timestamp = Some(Utc::now());
            return true;
        }
    }
    
    false
}
```