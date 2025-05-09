use actix_web::{web, HttpRequest, HttpResponse, Scope};
use validator::Validate;

use crate::errors::AuthError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::{
    DisableMfaRequest, EnableMfaRequest, LoginRequest, LogoutRequest, MfaLoginRequest,
    MfaRecoveryRequest, PasswordResetConfirmRequest, PasswordResetRequest, RefreshTokenRequest,
    RegisterRequest, VerifyEmailRequest, VerifyMfaRequest,
};
use crate::services::auth::AuthService;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(register)
            .service(login)
            .service(mfa_login)
            .service(refresh_token)
            .service(logout)
            .service(logout_all)
            .service(verify_email)
            .service(resend_verification_email)
            .service(password_reset)
            .service(password_reset_confirm)
            .service(mfa_setup)
            .service(mfa_enable)
            .service(mfa_disable)
            .service(mfa_recovery_codes)
            .service(mfa_verify)
            .service(mfa_recovery),
    );
}

#[actix_web::post("/register")]
async fn register(
    auth_service: web::Data<AuthService>,
    register_data: web::Json<RegisterRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, AuthError> {
    register_data.validate()?;
    
    let ip = req.connection_info().realip_remote_addr()
        .map(|s| s.to_string());
    
    let user_agent = req.headers().get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    
    let response = auth_service
        .register(register_data.into_inner(), ip, user_agent)
        .await?;
    
    Ok(HttpResponse::Created().json(response))
}

#[actix_web::post("/login")]
async fn login(
    auth_service: web::Data<AuthService>,
    login_data: web::Json<LoginRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, AuthError> {
    login_data.validate()?;
    
    let ip = req.connection_info().realip_remote_addr()
        .map(|s| s.to_string());
    
    let user_agent = req.headers().get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    
    let response = auth_service
        .login(login_data.into_inner(), ip, user_agent)
        .await?;
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::post("/mfa-login")]
async fn mfa_login(
    auth_service: web::Data<AuthService>,
    login_data: web::Json<MfaLoginRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, AuthError> {
    let ip = req.connection_info().realip_remote_addr()
        .map(|s| s.to_string());
    
    let user_agent = req.headers().get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    
    let response = auth_service
        .mfa_login(login_data.into_inner(), ip, user_agent)
        .await?;
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::post("/refresh-token")]
async fn refresh_token(
    auth_service: web::Data<AuthService>,
    refresh_data: web::Json<RefreshTokenRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, AuthError> {
    let ip = req.connection_info().realip_remote_addr()
        .map(|s| s.to_string());
    
    let user_agent = req.headers().get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    
    let response = auth_service
        .refresh_token(refresh_data.into_inner(), ip, user_agent)
        .await?;
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::post("/logout")]
async fn logout(
    auth_service: web::Data<AuthService>,
    user: Option<web::ReqData<AuthenticatedUser>>,
    logout_data: web::Json<LogoutRequest>,
) -> Result<HttpResponse, AuthError> {
    let user_id = user.map(|u| u.user_id);
    
    let response = auth_service
        .logout(logout_data.into_inner(), user_id)
        .await?;
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::post("/logout-all")]
async fn logout_all(
    auth_service: web::Data<AuthService>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<HttpResponse, AuthError> {
    let response = auth_service
        .logout_all(user.user_id)
        .await?;
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::post("/verify-email")]
async fn verify_email(
    auth_service: web::Data<AuthService>,
    verify_data: web::Json<VerifyEmailRequest>,
) -> Result<HttpResponse, AuthError> {
    let response = auth_service
        .verify_email(verify_data.into_inner())
        .await?;
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::post("/resend-verification-email")]
async fn resend_verification_email(
    auth_service: web::Data<AuthService>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<HttpResponse, AuthError> {
    let response = auth_service
        .resend_verification_email(user.user_id)
        .await?;
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::post("/password-reset")]
async fn password_reset(
    auth_service: web::Data<AuthService>,
    reset_data: web::Json<PasswordResetRequest>,
) -> Result<HttpResponse, AuthError> {
    reset_data.validate()?;
    
    let response = auth_service
        .password_reset_request(reset_data.into_inner())
        .await?;
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::post("/password-reset-confirm")]
async fn password_reset_confirm(
    auth_service: web::Data<AuthService>,
    confirm_data: web::Json<PasswordResetConfirmRequest>,
) -> Result<HttpResponse, AuthError> {
    confirm_data.validate()?;
    
    let response = auth_service
        .password_reset_confirm(confirm_data.into_inner())
        .await?;
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::get("/mfa-setup")]
async fn mfa_setup(
    auth_service: web::Data<AuthService>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<HttpResponse, AuthError> {
    let response = auth_service.mfa_setup(user.user_id).await?;
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::post("/mfa-enable")]
async fn mfa_enable(
    auth_service: web::Data<AuthService>,
    user: web::ReqData<AuthenticatedUser>,
    enable_data: web::Json<EnableMfaRequest>,
) -> Result<HttpResponse, AuthError> {
    enable_data.validate()?;
    
    let response = auth_service
        .mfa_enable(user.user_id, enable_data.into_inner())
        .await?;
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::post("/mfa-disable")]
async fn mfa_disable(
    auth_service: web::Data<AuthService>,
    user: web::ReqData<AuthenticatedUser>,
    disable_data: web::Json<DisableMfaRequest>,
) -> Result<HttpResponse, AuthError> {
    disable_data.validate()?;
    
    let response = auth_service
        .mfa_disable(user.user_id, disable_data.into_inner())
        .await?;
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::post("/mfa-verify")]
async fn mfa_verify(
    auth_service: web::Data<AuthService>,
    verify_data: web::Json<VerifyMfaRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, AuthError> {
    verify_data.validate()?;
    
    let ip = req.connection_info().realip_remote_addr()
        .map(|s| s.to_string());
    
    let user_agent = req.headers().get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    
    let response = auth_service
        .mfa_verify(verify_data.into_inner(), ip, user_agent)
        .await?;
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::get("/mfa-recovery-codes")]
async fn mfa_recovery_codes(
    auth_service: web::Data<AuthService>,
    user: web::ReqData<AuthenticatedUser>,
) -> Result<HttpResponse, AuthError> {
    let response = auth_service.mfa_recovery_codes(user.user_id).await?;
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::post("/mfa-recovery")]
async fn mfa_recovery(
    auth_service: web::Data<AuthService>,
    recovery_data: web::Json<MfaRecoveryRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, AuthError> {
    recovery_data.validate()?;
    
    let ip = req.connection_info().realip_remote_addr()
        .map(|s| s.to_string());
    
    let user_agent = req.headers().get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    
    let response = auth_service
        .mfa_recovery(recovery_data.into_inner(), ip, user_agent)
        .await?;
    
    Ok(HttpResponse::Ok().json(response))
}
