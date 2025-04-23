use axum::{Router, routing::{get, post, put, delete}};
use crate::handlers::*;

pub fn create_routes() -> Router {
    Router::new()
        // REST API
        .route("/api/docs", get(list_docs))
        .route("/api/docs/:name", get(get_doc))
        .route("/api/docs/:name", post(create_doc))
        .route("/api/docs/:name", put(edit_doc))
        .route("/api/docs/:name", delete(delete_doc))
        .route("/api/docs/check/:name", get(check_doc_exists))
        .route("/api/tags/:name", get(get_tags).post(update_tags))
        // Web 뷰어
        .route("/", get(redirect_to_index))
        .route("/docs", get(render_doc_list))
        .route("/templates", get(render_template_list))
        .route("/search", get(render_search_results))
        .route("/create", get(create_doc_page))
        .route("/edit/:name", get(edit_doc_page))
        .route("/:name", get(render_doc_page))
}