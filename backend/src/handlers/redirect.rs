use axum::response::Redirect;

pub async fn redirect_to_login() -> Redirect {
    Redirect::temporary("/login")
}

pub async fn redirect_to_index() -> Redirect {
    Redirect::temporary("/index")
}
