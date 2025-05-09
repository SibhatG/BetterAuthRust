use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

#[derive(Serialize)]
struct User {
    id: String,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "version": "0.1.0"
    }))
}

#[get("/users")]
async fn get_users() -> impl Responder {
    let users = vec![
        User {
            id: Uuid::new_v4().to_string(),
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        },
        User {
            id: Uuid::new_v4().to_string(),
            name: "Jane Smith".to_string(),
            email: "jane@example.com".to_string(),
        },
    ];
    
    HttpResponse::Ok().json(users)
}

#[post("/users")]
async fn create_user(user: web::Json<CreateUserRequest>) -> impl Responder {
    let new_user = User {
        id: Uuid::new_v4().to_string(),
        name: user.name.clone(),
        email: user.email.clone(),
    };
    
    HttpResponse::Created().json(new_user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("Starting server at 0.0.0.0:5000");
    
    // Start HTTP server
    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(get_users)
            .service(create_user)
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
