
use axum::{http::Request, response::{Html, IntoResponse, Redirect}, Json};
use crate::auth::{extract_valid_token, login, logout, signup, LoginRequest, SignUpRequest};
use crate::handlers::html_render::{render_login_page_html, render_signup_page_html};

pub async fn handle_login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    login(payload).await
}

pub async fn handle_signup(Json(payload): Json<SignUpRequest>) -> impl IntoResponse {
    signup(payload).await
}

pub async fn handle_logout() -> impl IntoResponse {
    logout().await
}

pub async fn render_login_page(req: Request<axum::body::Body>) -> impl IntoResponse {
    if extract_valid_token(req.headers()).is_some() {
        return Redirect::to("/index").into_response();
    }

    Html(render_login_page_html()).into_response()
}

pub async fn render_signup_page() -> Html<String> {
    Html(render_signup_page_html())
}