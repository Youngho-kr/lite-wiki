use axum::{http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::auth::{build_jwt_cookie, create_jwt, verify_password};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
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

    let token = create_jwt(&user.username).unwrap();
    let cookie = build_jwt_cookie(&token);

    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    (headers, Json(LoginResponse { token })).into_response()
}
