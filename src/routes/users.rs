use actix_web::{web, HttpResponse, Scope};
use validator::Validate;

use crate::errors::AuthError;
use crate::middleware::auth::AuthenticatedUser;
use crate::services::auth::AuthService;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(get_me)
            .service(get_sessions)
            .service(revoke_session),
    );
}

#[actix_web::get("/me")]
async fn get_me(
    auth_service: web::Data<AuthService>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<HttpResponse, AuthError> {
    let response = auth_service.get_user(user.user_id).await?;
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::get("/sessions")]
async fn get_sessions(
    auth_service: web::Data<AuthService>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<HttpResponse, AuthError> {
    let response = auth_service.get_sessions(user.user_id).await?;
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::delete("/sessions/{session_id}")]
async fn revoke_session(
    auth_service: web::Data<AuthService>,
    user: web::ReqData<AuthenticatedUser>,
    session_id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, AuthError> {
    let response = auth_service
        .revoke_session(user.user_id, *session_id)
        .await?;
    
    Ok(HttpResponse::Ok().json(response))
}
