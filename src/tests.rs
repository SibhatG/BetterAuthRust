#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App, web};
    use serde_json::json;
    use uuid::Uuid;
    use std::collections::HashMap;
    use std::sync::Mutex;

    #[actix_web::test]
    async fn test_register_login_flow() {
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
        let username = format!("testuser_{}", Uuid::new_v4());
        let email = format!("{}@example.com", username);
        
        let register_payload = json!({
            "username": username,
            "email": email,
            "password": "Test123!",
            "password_confirmation": "Test123!"
        });

        let register_req = test::TestRequest::post()
            .uri("/api/auth/register")
            .set_json(&register_payload)
            .to_request();

        let register_resp = test::call_service(&app, register_req).await;
        assert_eq!(register_resp.status(), 201);

        // Test login endpoint with correct credentials
        let login_payload = json!({
            "username_or_email": username,
            "password": "Test123!"
        });

        let login_req = test::TestRequest::post()
            .uri("/api/auth/login")
            .set_json(&login_payload)
            .to_request();

        let login_resp = test::call_service(&app, login_req).await;
        assert_eq!(login_resp.status(), 200);

        // Test login with invalid password
        let invalid_login_payload = json!({
            "username_or_email": username,
            "password": "WrongPassword"
        });

        let invalid_login_req = test::TestRequest::post()
            .uri("/api/auth/login")
            .set_json(&invalid_login_payload)
            .to_request();

        let invalid_login_resp = test::call_service(&app, invalid_login_req).await;
        assert_eq!(invalid_login_resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new().service(health_check)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        
        assert_eq!(resp.status(), 200);
        
        let body = test::read_body(resp).await;
        let json_body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(json_body["status"], "ok");
    }
}