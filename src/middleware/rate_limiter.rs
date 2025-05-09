use std::collections::HashMap;
use std::future::{ready, Ready};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use futures::Future;

use crate::errors::AuthError;

// Simple in-memory rate limiter
pub struct RateLimiter {
    max_requests: u32,
    window_duration: u64,
    cache: Arc<Mutex<HashMap<String, (u32, Instant)>>>,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_duration: u64) -> Self {
        RateLimiter {
            max_requests,
            window_duration,
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddleware {
            service,
            max_requests: self.max_requests,
            window_duration: self.window_duration,
            cache: self.cache.clone(),
        }))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
    max_requests: u32,
    window_duration: u64,
    cache: Arc<Mutex<HashMap<String, (u32, Instant)>>>,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
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
        // Get client IP
        let ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();

        // Clean up expired entries
        {
            let mut cache = self.cache.lock().unwrap();
            cache.retain(|_, (_, timestamp)| {
                timestamp.elapsed() < Duration::from_secs(self.window_duration)
            });
        }

        // Check if client exceeds rate limit
        let now = Instant::now();
        let mut exceeded = false;

        {
            let mut cache = self.cache.lock().unwrap();
            let entry = cache.entry(ip).or_insert((0, now));

            // Reset counter if window has elapsed
            if entry.1.elapsed() >= Duration::from_secs(self.window_duration) {
                *entry = (1, now);
            } else {
                // Increment counter
                entry.0 += 1;
                // Check if rate limit exceeded
                if entry.0 > self.max_requests {
                    exceeded = true;
                }
            }
        }

        if exceeded {
            return Box::pin(async move {
                Err(AuthError::RateLimitExceeded.into())
            });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
