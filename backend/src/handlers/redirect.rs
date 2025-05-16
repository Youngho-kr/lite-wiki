use axum::response::Redirect;

use crate::config::current_redirect_page;

pub async fn redirect_to_root() -> Redirect {
    Redirect::to(current_redirect_page().trim_start_matches('/'))
}

pub fn redirec_to_page(name: &str) -> Redirect {
    Redirect::to(name)
}