use axum::{middleware, routing::{delete, get, post}, Router};
use tower_http::services::ServeDir;
use crate::{auth::{require_admin, require_admin_or_redirect, require_jwt, require_jwt_or_redirect}, handlers::*};

pub fn create_routes() -> Router {
    Router::new()
        // css
        .nest_service("/static", ServeDir::new("static"))
        // Auth
        .route("/api/login", post(handle_login))
        .route("/api/signup", post(handle_signup))
        .route("/api/logout", get(handle_logout))
        .route("/api/change-password", post(handle_change_password))
        // Admin
        .route("/api/admin", post(save_admin_setting).layer(middleware::from_fn(require_admin)))
        // REST API
        .route("/api/docs", get(list_docs).layer(middleware::from_fn(require_jwt)))
        .route("/api/docs/:name", get(get_doc).layer(middleware::from_fn(require_jwt)))
        .route("/api/docs/:name", post(post_doc).layer(middleware::from_fn(require_jwt)))
        .route("/api/docs/:name", delete(delete_doc).layer(middleware::from_fn(require_jwt)))
        .route("/api/docs/check/:name", get(check_doc_exists).layer(middleware::from_fn(require_jwt)))
        .route("/api/tags/:name", get(get_tags).layer(middleware::from_fn(require_jwt)))
        // Web 뷰어
        .route("/", get(redirect_to_root))
        .route("/login", get(render_login_page))
        .route("/signup", get(render_signup_page))
        .route("/user_info", get(render_user_info_page).layer(middleware::from_fn(require_jwt_or_redirect)))
        .route("/admin", get(render_admin_page).layer(middleware::from_fn(require_admin_or_redirect)))
        .route("/docs", get(render_doc_list).layer(middleware::from_fn(require_jwt_or_redirect)))
        // .route("/templates", get(render_template_list).layer(middleware::from_fn(require_jwt_or_redirect)))
        .route("/search", get(render_search_results).layer(middleware::from_fn(require_jwt_or_redirect)))
        .route("/create", get(create_doc_page).layer(middleware::from_fn(require_jwt_or_redirect)))
        .route("/tags", get(render_all_tags).layer(middleware::from_fn(require_jwt_or_redirect)))
        .route("/random", get(random_page).layer(middleware::from_fn(require_jwt_or_redirect)))
        .route("/:name", get(render_doc_page).layer(middleware::from_fn(require_jwt_or_redirect)))
        .route("/edit/:name", get(edit_doc_page).layer(middleware::from_fn(require_jwt_or_redirect)))
        .route("/tags/:name", get(render_search_tags).layer(middleware::from_fn(require_jwt_or_redirect)))
}