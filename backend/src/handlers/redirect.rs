use axum::response::Redirect;

pub async fn redirect_to_index() -> Redirect {
    Redirect::temporary("/index")
}
