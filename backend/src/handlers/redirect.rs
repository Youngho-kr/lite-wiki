use axum::response::Redirect;

use crate::config::get_redirect_page;

pub async fn redirect_to_root() -> Redirect {
    Redirect::to(&get_redirect_page())
}