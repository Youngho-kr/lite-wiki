
use axum::{ response::{Html, IntoResponse}, Json};
use crate::auth::{login, signup, LoginRequest, SignUpRequest};
use crate::handlers::html_render::{render_login_page_html, render_signup_page_html};

pub async fn handle_login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    login(payload).await
}

pub async fn handle_signup(Json(payload): Json<SignUpRequest>) -> impl IntoResponse {
    signup(payload).await
}

pub async fn render_login_page() -> Html<String> {
    Html(render_login_page_html())
}

pub async fn render_signup_page() -> Html<String> {
    Html(render_signup_page_html())
}