use axum::{
    extract::Request,
    middleware::Next,
    response::Response
};
use crate::auth::jwt::verify_jwt;
use crate::config::env::Config;
use crate::error::AppError;

pub async fn auth_guard(
    req:Request,
    next:Next,
)->Result<Response,AppError> {
    let config=Config::init();
    let auth_header=req.headers().get("Authorization")
        .and_then(| header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized)?;
    let claims=verify_jwt(auth_header, &config.jwt_secret).map_err(|_| AppError::Forbidden)?;
    if claims.role=="admin"{
        return Err(AppError::Forbidden);
    }
    Ok(next.run(req).await)

    
}