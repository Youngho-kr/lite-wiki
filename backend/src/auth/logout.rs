use axum::{http::header::SET_COOKIE, response::IntoResponse};

use crate::{auth::build_jwt_removal_cooke, handlers::redirect_to_root};

pub async fn logout() -> impl IntoResponse {
    let remove_cookie = build_jwt_removal_cooke();

    (
        [(SET_COOKIE, remove_cookie.to_string())],
        redirect_to_root()
    )
}