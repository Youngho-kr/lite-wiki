use axum::response::Redirect;

use crate::config::current_redirect_page;

pub async fn redirect_to_root() -> Redirect {
    Redirect::to(&current_redirect_page())
}