use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{auth::{respond_with_token_headers, verify_password}, handlers::redirect_to_root};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login(payload: LoginRequest) -> impl IntoResponse {
    let user = match verify_password(&payload.username, &payload.password) {
        Some(u) => u,
        None => {
            info!("로그인 실패: 비밀번호 오류 ({})", payload.username);
            return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response();
        }
    };

    if !user.is_authorized {
        info!("로그인 실패: 권한 허용되지 않은 사용자 {}", payload.username);
        return (StatusCode::FORBIDDEN, "Waiting for approval").into_response()
    }

    info!("로그인 성공: {}", user.username);

    let headers = respond_with_token_headers(&user.username);

    headers.into_response()
}
