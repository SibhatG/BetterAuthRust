use std::future::{ready, Ready};
use std::pin::Pin;

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use futures::Future;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AuthError;
use crate::utils::jwt::{decode_jwt, JwtClaims};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub is_admin: bool,
}

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract authorization header
        let auth_header = req.headers().get("Authorization");

        if auth_header.is_none() {
            return Box::pin(async move {
                Err(AuthError::InvalidToken.into())
            });
        }

        let auth_header = auth_header.unwrap().to_str().unwrap_or("");

        if !auth_header.starts_with("Bearer ") {
            return Box::pin(async move {
                Err(AuthError::InvalidToken.into())
            });
        }

        let token = auth_header[7..].to_string();

        // Create a clone of the request to move into the future
        let mut req_clone = req.clone();

        // Decode JWT and process request
        Box::pin(async move {
            match decode_jwt::<JwtClaims>(&token) {
                Ok(claims) => {
                    let user = AuthenticatedUser {
                        user_id: claims.sub,
                        is_admin: claims.is_admin,
                    };

                    req_clone.extensions_mut().insert(user);
                    let fut = self.service.call(req_clone);
                    let res = fut.await?;
                    Ok(res)
                }
                Err(err) => Err(err.into()),
            }
        })
    }
}

pub struct AdminMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AdminMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AdminMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminMiddlewareService { service }))
    }
}

pub struct AdminMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AdminMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Check if user is authenticated
        let user_option = req.extensions().get::<AuthenticatedUser>().cloned();
        
        if let Some(user) = user_option {
            if user.is_admin {
                let fut = self.service.call(req);
                return Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                });
            }
        }

        Box::pin(async move {
            Err(AuthError::PermissionDenied.into())
        })
    }
}
