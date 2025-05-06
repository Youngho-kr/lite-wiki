use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::Json;
use serde::Deserialize;

use crate::auth::{load_users, save_users, AuthUser};
use crate::config::{get_redirect_page, save_redirect_setting};
use crate::handlers::html_render::render_admin_page_html;

#[derive(Deserialize)]
pub struct AdminUpdatePayload {
    users: Vec<UserUpdate>,
    redirect_page: String,
}

#[derive(Deserialize)]
pub struct UserUpdate {
    username: String,
    is_authorized: bool,
}

pub async fn render_admin_page(
    AuthUser(username): AuthUser
) -> Html<String> {
    let users = load_users().unwrap_or_default();
    Html(render_admin_page_html(&users, &get_redirect_page(), &username))
}

pub async fn save_admin_setting(
    Json(payload): Json<AdminUpdatePayload>,
) -> impl IntoResponse {
    let mut users = load_users().unwrap_or_default();

    for user in &mut users {
        if let Some(update) = payload.users.iter().find(|u| u.username == user.username) {
            user.is_authorized = update.is_authorized;
        }
    }

    if let Err(_) = save_users(&users) {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    if let Err(_) = save_redirect_setting(&payload.redirect_page) {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    let redirect = payload.redirect_page;
    save_redirect_setting(&redirect).unwrap();

    StatusCode::OK
}
