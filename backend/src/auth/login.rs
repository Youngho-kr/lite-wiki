use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::{auth::load_users, config::JWT_SECRET};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn login(payload: LoginRequest) -> impl IntoResponse {
    let users = load_users().unwrap_or_default();
    let user = match users.iter().find(|u| u.username == payload.username) {
        Some(u) => u,
        None => return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
    };

    if !user.is_authorized {
        return (StatusCode::FORBIDDEN, "Waiting for approval").into_response()
    }

    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
    if Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash).is_err() {
        return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response();
    }

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user.username.clone(),
        exp: expiration,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.clone().as_bytes()))
        .unwrap();

    Json(LoginResponse { token }).into_response()
}
