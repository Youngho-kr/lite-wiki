use axum::{middleware, routing::{delete, get, post, put}, Router};
use crate::{auth::require_jwt, handlers::*};

pub fn create_routes() -> Router {
    Router::new()
        // Auth
        .route("/api/login", post(handle_login))
        .route("/api/signup", post(handle_signup))
        // REST API
        .route("/api/docs", get(list_docs).layer(middleware::from_fn(require_jwt)))
        .route("/api/docs/:name", get(get_doc).layer(middleware::from_fn(require_jwt)))
        .route("/api/docs/:name", post(create_doc).layer(middleware::from_fn(require_jwt)))
        .route("/api/docs/:name", put(edit_doc).layer(middleware::from_fn(require_jwt)))
        .route("/api/docs/:name", delete(delete_doc).layer(middleware::from_fn(require_jwt)))
        .route("/api/docs/check/:name", get(check_doc_exists).layer(middleware::from_fn(require_jwt)))
        .route("/api/tags/:name", get(get_tags).post(update_tags).layer(middleware::from_fn(require_jwt)))
        // Web 뷰어
        .route("/", get(redirect_to_index))
        .route("/login", get(render_login_page))
        .route("/signup", get(render_signup_page))
        .route("/docs", get(render_doc_list).layer(middleware::from_fn(require_jwt)))
        .route("/templates", get(render_template_list).layer(middleware::from_fn(require_jwt)))
        .route("/search", get(render_search_results).layer(middleware::from_fn(require_jwt)))
        .route("/create", get(create_doc_page).layer(middleware::from_fn(require_jwt)))
        .route("/tags", get(render_all_tags).layer(middleware::from_fn(require_jwt)))
        .route("/edit/:name", get(edit_doc_page).layer(middleware::from_fn(require_jwt)))
        .route("/:name", get(render_doc_page).layer(middleware::from_fn(require_jwt)))
        .route("/tag/:name", get(render_search_tags).layer(middleware::from_fn(require_jwt)))
}