use axum::{extract::Request, middleware::Next, response::Response};
use core_crate::AppResult;
use sea_query::ExprTrait;
use tracing::{debug, info};
use dotenv::dotenv;

use crate::utils::verify_jwt_token;
#[derive(Debug,Clone)]
pub struct AuthUser{
    pub user_id:i32,
}
pub async fn mw_auth(req: Request, next: Next) -> AppResult<Response> {
    debug!("->> MIDDLEWARE AUTH");
    Ok(next.run(req).await)
}

pub async fn mw_auth_with_secret_key(
    req: Request,
    next: Next,
) -> AppResult<Response> {
    let env_key = std::env::var("SECRET_KEY").ok().filter(|s| !s.is_empty());
    let Some(env_key) = env_key else {
        debug!("->> SECRET KEY NOT FOUND IN ENV");
        return Err(core_crate::error::AppError::Unauthorized);
    };

    let header_key = req.headers().get("Authorization").and_then(|hv| hv.to_str().ok()).map(|s| s.trim()).map(|s| s.strip_prefix("Bearer").unwrap_or_default().trim());
    info!("Received Header Key: {:?}", header_key);
    let Some(header_key) = header_key else {
        debug!("->> AUTHORIZATION HEADER NOT FOUND/INVALID");
        return Err(core_crate::error::AppError::Unauthorized);
    };

    if header_key == env_key {
        debug!("->> MIDDLEWARE AUTH SUCCESS");
        Ok(next.run(req).await)
    } else {
        debug!("->> SECRET KEY MISMATCH");
        Err(core_crate::error::AppError::Unauthorized)
    }
}
pub async fn mw_auth_with_jwt(
  mut  req: Request,
    next: Next,
) -> AppResult<Response> {
    debug!("->> MIDDLEWARE AUTH WITH JWT");
    let header_key = req.headers().get("Authorization").and_then(|hv| hv.to_str().ok()).map(|s| s.trim()).map(|s| s.strip_prefix("Bearer").unwrap_or_default().trim());
    //verify jwt token
    let is_valid=verify_jwt_token::<String>(header_key.unwrap_or_default());
        match is_valid {
        Ok(_user_id) => {
            debug!("->> JWT TOKEN VALID");
            req.extensions_mut().insert(AuthUser{user_id: _user_id.parse::<i32>().unwrap_or_default()});
            Ok(next.run(req).await)
        },
        Err(_) => {
            debug!("->> JWT TOKEN INVALID");
            Err(core_crate::error::AppError::Unauthorized)
        }
}
}