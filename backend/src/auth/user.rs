use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};
use serde::{Deserialize, Serialize};

use crate::auth::decode_jwt;

use super::extract_valid_token;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub is_authorized: bool,
}

#[derive(Clone, Debug)]
pub struct AuthUser(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where 
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;
        let Some(token) = extract_valid_token(headers) else {
            return Err(StatusCode::UNAUTHORIZED);
        };

        let username = decode_jwt(&token).map_err(|_| StatusCode::UNAUTHORIZED)?;
        Ok(AuthUser(username))
    }
}