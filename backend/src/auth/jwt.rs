use axum::{
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use headers::HeaderMapExt;
use headers::Cookie as HeaderCookie;
use jsonwebtoken::{decode, encode, errors::Error as JwtError, Header, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use cookie::{time, Cookie, SameSite};

use crate::config::JWT_SECRET;

use super::load_users;

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

pub fn build_jwt_cookie(token: &str) -> Cookie {
    let secure = false; // 로컬 개발 시 false, 배포 시 true

    let mut cookie = Cookie::new("token", token.to_string());
    cookie.set_path("/");
    cookie.set_max_age(time::Duration::hours(1));
    cookie.set_http_only(true);
    cookie.set_secure(secure);
    cookie.set_same_site(SameSite::Lax);
    cookie
}

pub fn build_jwt_removal_cooke() -> Cookie<'static> {
    let secure = false; // 로컬 개발 시 false, 배포 시 true

    let mut cookie = Cookie::new("token", "");
    cookie.set_path("/");
    cookie.set_max_age(time::Duration::seconds(0));
    cookie.set_http_only(true);
    cookie.set_secure(secure);
    cookie.set_same_site(SameSite::Lax);
    cookie
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

pub async fn require_jwt_or_redirect(req: Request<axum::body::Body>, next: Next) -> Result<Response, StatusCode> {
    let cookies = req.headers().typed_get::<HeaderCookie>();
    let token = match cookies.and_then(|c| c.get("token").map(|s| s.to_string())) {
        Some(t) => t,
        None => return Ok(Redirect::to("/login").into_response()),
    };

    if decode_jwt(&token).is_err() {
        return Ok(Redirect::to("/login").into_response());
    }

    Ok(next.run(req).await)
}

pub async fn require_admin(req: Request<axum::body::Body>, next: Next) -> Result<Response, StatusCode> {
    let cookies = req.headers().typed_get::<HeaderCookie>();
    let token = match cookies.and_then(|c| c.get("token").map(|s| s.to_string())) {
        Some(t) => t,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let username = match decode_jwt(&token) {
        Ok(name) => name,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let users = load_users().unwrap_or_default();
    let user = users.iter().find(|u| u.username == username);

    match user {
        Some(u) if u.is_admin => Ok(next.run(req).await),
        _ => return Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn require_admin_or_redirect(req: Request<axum::body::Body>, next: Next) -> Result<Response, StatusCode> {
    let cookies = req.headers().typed_get::<HeaderCookie>();
    let token = match cookies.and_then(|c| c.get("token").map(|s| s.to_string())) {
        Some(t) => t,
        None => return Ok(Redirect::to("/login").into_response()),
    };

    let username = match decode_jwt(&token) {
        Ok(name) => name,
        Err(_) => return Ok(Redirect::to("/").into_response()),
    };

    let users = load_users().unwrap_or_default();
    let user = users.iter().find(|u| u.username == username);

    match user {
        Some(u) if u.is_admin => Ok(next.run(req).await),
        _ => Ok(Redirect::to("/").into_response()),
    }
}

pub fn extract_valid_token(headers: &HeaderMap) -> Option<String> {
    if let Some(cookie_header) = headers.typed_get::<HeaderCookie>() {
        if let Some(token) = cookie_header.get("token") {
            match decode_jwt(token) {
                Ok(_) => return Some(token.to_string()),
                Err(e) => tracing::warn!("유효하지 않은 JWT: {}", e),
            }
        }
    }
    None
}