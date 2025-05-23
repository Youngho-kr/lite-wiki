use axum::response::Redirect;

use crate::config::current_redirect_page;

pub async fn handle_root() -> Redirect {
    redirect_to_root()
}

pub fn redirect_to_root() -> Redirect {
    Redirect::to(&current_redirect_page())
}

pub fn redirec_to_page(name: &str) -> Redirect {
    Redirect::to(&format!("/{}", name))
}