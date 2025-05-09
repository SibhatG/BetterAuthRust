pub mod auth_types {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;
    use std::collections::HashMap;
    use std::sync::Mutex;

    // Simplified model structs for demonstration
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct User {
        pub id: Uuid,
        pub username: String,
        pub email: String,
        pub password_hash: String,
        pub is_email_verified: bool,
        pub mfa_enabled: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Session {
        pub id: Uuid,
        pub user_id: Uuid,
        pub refresh_token: String,
        pub expires_at: chrono::DateTime<chrono::Utc>,
    }

    // In-memory database for development
    pub struct AppState {
        pub users: Mutex<HashMap<Uuid, User>>,
        pub sessions: Mutex<HashMap<Uuid, Session>>,
    }

    // Request and response structs
    #[derive(Debug, Deserialize)]
    pub struct RegisterRequest {
        pub username: String,
        pub email: String,
        pub password: String,
        pub password_confirmation: String,
    }

    #[derive(Debug, Serialize)]
    pub struct UserResponse {
        pub id: Uuid,
        pub username: String,
        pub email: String,
        pub is_email_verified: bool,
        pub mfa_enabled: bool,
    }

    #[derive(Debug, Serialize)]
    pub struct RegisterResponse {
        pub user: UserResponse,
        pub message: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct LoginRequest {
        pub username_or_email: String,
        pub password: String,
    }

    #[derive(Debug, Serialize)]
    pub struct LoginResponse {
        pub access_token: String,
        pub refresh_token: String,
        pub token_type: String,
        pub expires_in: u64,
        pub user: UserResponse,
    }

    #[derive(Debug, Serialize)]
    pub struct ErrorResponse {
        pub status: String,
        pub code: String,
        pub message: String,
    }
}

pub mod auth_utils {
    // Helper functions for password hashing (simplified for demo)
    pub fn hash_password(password: &str) -> String {
        // In a real app, use bcrypt or argon2
        format!("hashed_{}", password)
    }

    pub fn verify_password(password: &str, hash: &str) -> bool {
        // In a real app, use bcrypt or argon2 verification
        hash == format!("hashed_{}", password)
    }
}

// Re-export for lib usage
pub use auth_types::*;
pub use auth_utils::*;

// Handler functions
use actix_web::{get, post, web, HttpResponse, Responder, Error, HttpRequest};
use serde_json::json;
use uuid::Uuid;

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "version": "0.1.0"
    }))
}

#[post("/api/auth/register")]
pub async fn register(data: web::Json<auth_types::RegisterRequest>, state: web::Data<auth_types::AppState>) -> Result<HttpResponse, Error> {
    let data = data.into_inner();
    
    // Validate input
    if data.password != data.password_confirmation {
        return Ok(HttpResponse::BadRequest().json(auth_types::ErrorResponse {
            status: "error".to_string(),
            code: "VALIDATION_ERROR".to_string(),
            message: "Passwords do not match".to_string(),
        }));
    }

    // Check if user exists
    let users = state.users.lock().unwrap();
    for user in users.values() {
        if user.username == data.username {
            return Ok(HttpResponse::BadRequest().json(auth_types::ErrorResponse {
                status: "error".to_string(),
                code: "USERNAME_EXISTS".to_string(),
                message: "Username already exists".to_string(),
            }));
        }
        if user.email == data.email {
            return Ok(HttpResponse::BadRequest().json(auth_types::ErrorResponse {
                status: "error".to_string(),
                code: "EMAIL_EXISTS".to_string(),
                message: "Email already exists".to_string(),
            }));
        }
    }
    drop(users);
    
    // Create new user
    let user_id = Uuid::new_v4();
    let user = auth_types::User {
        id: user_id,
        username: data.username,
        email: data.email,
        password_hash: auth_utils::hash_password(&data.password),
        is_email_verified: false,
        mfa_enabled: false,
    };
    
    // Save user to "database"
    let mut users = state.users.lock().unwrap();
    users.insert(user_id, user.clone());
    
    // Return response
    Ok(HttpResponse::Created().json(auth_types::RegisterResponse {
        user: auth_types::UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            is_email_verified: user.is_email_verified,
            mfa_enabled: user.mfa_enabled,
        },
        message: "User registered successfully. Please check your email to verify your account.".to_string(),
    }))
}

#[post("/api/auth/login")]
pub async fn login(data: web::Json<auth_types::LoginRequest>, state: web::Data<auth_types::AppState>) -> Result<HttpResponse, Error> {
    let data = data.into_inner();
    
    // Find user by username or email
    let users = state.users.lock().unwrap();
    let mut user_found = None;
    
    for user in users.values() {
        if user.username == data.username_or_email || user.email == data.username_or_email {
            user_found = Some(user.clone());
            break;
        }
    }
    drop(users);
    
    // Check if user exists
    let user = match user_found {
        Some(user) => user,
        None => {
            return Ok(HttpResponse::Unauthorized().json(auth_types::ErrorResponse {
                status: "error".to_string(),
                code: "INVALID_CREDENTIALS".to_string(),
                message: "Invalid credentials".to_string(),
            }))
        }
    };
    
    // Verify password
    if !auth_utils::verify_password(&data.password, &user.password_hash) {
        return Ok(HttpResponse::Unauthorized().json(auth_types::ErrorResponse {
            status: "error".to_string(),
            code: "INVALID_CREDENTIALS".to_string(),
            message: "Invalid credentials".to_string(),
        }));
    }
    
    // Generate tokens (in a real app, use JWT)
    let access_token = Uuid::new_v4().to_string();
    let refresh_token = Uuid::new_v4().to_string();
    
    // Create session
    let session_id = Uuid::new_v4();
    let session = auth_types::Session {
        id: session_id,
        user_id: user.id,
        refresh_token: refresh_token.clone(),
        expires_at: chrono::Utc::now() + chrono::Duration::days(7),
    };
    
    // Save session
    let mut sessions = state.sessions.lock().unwrap();
    sessions.insert(session_id, session);
    
    // Return response
    Ok(HttpResponse::Ok().json(auth_types::LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: 3600,
        user: auth_types::UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            is_email_verified: user.is_email_verified,
            mfa_enabled: user.mfa_enabled,
        },
    }))
}

#[get("/api/users/me")]
pub async fn get_current_user(req: HttpRequest, state: web::Data<auth_types::AppState>) -> Result<HttpResponse, Error> {
    // In a real app, extract user details from JWT token
    // For this demo, just return an error saying auth is required
    
    Ok(HttpResponse::Unauthorized().json(auth_types::ErrorResponse {
        status: "error".to_string(),
        code: "AUTHENTICATION_ERROR".to_string(),
        message: "Authentication required".to_string(),
    }))
}

// Actual binary main function
use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use actix_web::http::header;
use dotenv::dotenv;
use log::info;
use std::collections::HashMap;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("Starting Better Auth server at 0.0.0.0:5000");
    
    // Create app state (in-memory database)
    let app_state = web::Data::new(auth_types::AppState {
        users: Mutex::new(HashMap::new()),
        sessions: Mutex::new(HashMap::new()),
    });
    
    // Start HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
            .max_age(3600);
        
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(health_check)
            .service(register)
            .service(login)
            .service(get_current_user)
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
