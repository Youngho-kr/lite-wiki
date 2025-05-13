use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::Json;
use serde::Deserialize;

use crate::auth::{list_all_users, update_user, AuthUser};
use crate::config::{current_redirect_page, update_redirect_page};
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
    let users = list_all_users().unwrap_or_default();
    Html(render_admin_page_html(&users, &current_redirect_page(), &username))
}

pub async fn save_admin_setting(
    Json(payload): Json<AdminUpdatePayload>,
) -> impl IntoResponse {
    for user in payload.users {
        update_user(&user.username, |u| { u.is_authorized = user.is_authorized; }).unwrap();
    }

    if let Err(_) = update_redirect_page(&payload.redirect_page) {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    let redirect = payload.redirect_page;
    update_redirect_page(&redirect).unwrap();

    StatusCode::OK
}
