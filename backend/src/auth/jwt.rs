use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use headers::HeaderMapExt;
use headers::Cookie as HeaderCookie;
use jsonwebtoken::{decode, encode, errors::Error as JwtError, Header, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use cookie::{Cookie, SameSite};

use crate::config::JWT_SECRET;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(username: &str) -> Result<String, JwtError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: username.to_string(),
        exp: expiration
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.clone().as_bytes()))
}

pub fn decode_jwt(token: &str) -> Result<String, JwtError> {
    let data = decode::<Claims>(
        token, 
        &DecodingKey::from_secret(JWT_SECRET.clone().as_bytes()), 
        &Validation::default()
    )?;

    Ok(data.claims.sub)
}

pub async fn require_jwt(req: Request<axum::body::Body>, next: Next) -> Result<Response, StatusCode> {

    let cookies = req.headers().typed_get::<HeaderCookie>();

    let token = match cookies.and_then(|c| c.get("token").map(|s| s.to_string())) {
        Some(t) => t,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if decode_jwt(&token).is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}

pub fn build_jwt_cookie(token: &str) -> Cookie {
    let secure = false; // 로컬 개발 시 false, 배포 시 true

    let mut cookie = Cookie::new("token", token.to_string());
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_secure(secure);
    cookie.set_same_site(SameSite::Lax);
    cookie
}