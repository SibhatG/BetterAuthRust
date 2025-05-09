use actix_web::{test, App, web};
use serde_json::json;
use std::sync::Mutex;
use std::collections::HashMap;
use uuid::Uuid;

// Import directly from crate root
mod common {
    pub use crate::auth_types;
    pub use crate::register;
    pub use crate::login;
}
use common::{auth_types, register, login};

#[actix_web::test]
async fn test_register_and_login() {
    // Setup test app state
    let app_state = web::Data::new(auth_types::AppState {
        users: Mutex::new(HashMap::new()),
        sessions: Mutex::new(HashMap::new()),
    });

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .service(register)
            .service(login),
    )
    .await;

    // Test register endpoint
    let register_payload = json!({
        "username": "testuser",
        "email": "testuser@example.com",
        "password": "Test123!",
        "password_confirmation": "Test123!"
    });

    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&register_payload)
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status(), 201);

    // Test login endpoint
    let login_payload = json!({
        "username_or_email": "testuser",
        "password": "Test123!"
    });

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_payload)
        .to_request();

    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), 200);
}

#[actix_web::test]
async fn test_invalid_login() {
    // Setup test app state
    let app_state = web::Data::new(auth_types::AppState {
        users: Mutex::new(HashMap::new()),
        sessions: Mutex::new(HashMap::new()),
    });

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .service(register)
            .service(login),
    )
    .await;

    // First register a user
    let register_payload = json!({
        "username": "testuser2",
        "email": "testuser2@example.com",
        "password": "Test123!",
        "password_confirmation": "Test123!"
    });

    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&register_payload)
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status(), 201);

    // Test login with invalid credentials
    let login_payload = json!({
        "username_or_email": "testuser2",
        "password": "WrongPassword"
    });

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_payload)
        .to_request();

    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), 401);
}