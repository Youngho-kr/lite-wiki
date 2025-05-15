use axum::response::Redirect;

use crate::config::{current_redirect_page, BASE_URL};

pub async fn redirect_to_root() -> Redirect {
    Redirect::to(&format!("{}{}", *BASE_URL, current_redirect_page()))
}

pub fn redirec_to_page(name: &str) -> Redirect {
    Redirect::to(&format!("{}/{}", *BASE_URL, name))
}