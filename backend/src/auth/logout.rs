use axum::{http::header::SET_COOKIE, response::{IntoResponse, Redirect}};

use crate::auth::build_jwt_removal_cooke;

pub async fn logout() -> impl IntoResponse {
    let remove_cookie = build_jwt_removal_cooke();

    (
        [(SET_COOKIE, remove_cookie.to_string())],
        Redirect::to("/"),
    )
}