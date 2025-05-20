
use axum::{http::{Request, StatusCode}, response::{Html, IntoResponse}, Json};
use crate::auth::{change_password, extract_valid_token, login, logout, signup, verify_password, AuthUser, ChangePasswordReqeust, LoginRequest, SignUpRequest};
use crate::handlers::html_render::{render_login_page_html, render_signup_page_html};

use super::{redirect_to_root, render_user_info_html};

pub async fn handle_login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    login(payload).await
}

pub async fn handle_signup(Json(payload): Json<SignUpRequest>) -> impl IntoResponse {
    signup(payload).await
}

pub async fn handle_logout() -> impl IntoResponse {
    logout().await
}

pub async fn handle_github_login() -> impl IntoResponse {

}

pub async fn handle_change_password(
    AuthUser(username): AuthUser,
    Json(payload): Json<ChangePasswordReqeust>,
) -> impl IntoResponse {
    let user = match verify_password(&username, &payload.current) {
        Some(u) => u,
        None => return StatusCode::UNAUTHORIZED,
    };

    match change_password(user, &payload.new) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn render_login_page(req: Request<axum::body::Body>) -> impl IntoResponse {
    if extract_valid_token(req.headers()).is_some() {
        return redirect_to_root().into_response();
    }

    Html(render_login_page_html()).into_response()
}

pub async fn render_signup_page() -> Html<String> {
    Html(render_signup_page_html())
}

pub async fn render_user_info_page(
    AuthUser(username): AuthUser
) -> Html<String> {
    Html(render_user_info_html(&username))
}