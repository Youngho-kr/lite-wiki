use argon2::{Argon2, PasswordHasher};
use axum::{http::StatusCode, response::IntoResponse};
use password_hash::{rand_core::OsRng, SaltString};
use serde::Deserialize;
use crate::auth::{User, add_user};

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub password: String,
}

pub async fn signup(payload: SignUpRequest) -> impl IntoResponse {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let new_user = User {
        username: payload.username,
        password_hash,
        is_admin: false,
        is_authorized: false,
    };

    add_user(new_user).unwrap();

    (StatusCode::CREATED, "User created").into_response()
}