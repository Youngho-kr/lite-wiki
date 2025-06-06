use std::collections::HashMap;

use axum::{
    extract::{Path, Query}, 
    response::{Html, IntoResponse}
};
use crate::{auth::AuthUser, handlers::html_render::*};
use crate::storage::{load_doc, load_doc_meta};

use super::redirec_to_page;

pub async fn render_doc_page(
    Path(title): Path<String>, 
    AuthUser(username): AuthUser
) -> impl IntoResponse {
    match load_doc(&title) {
        Ok(md_content) => {
            let meta = load_doc_meta(&title).unwrap_or_default();
            Html(render_viewer_html(&title, &md_content, &meta.tags, &meta.history, &username)).into_response()
        }
        Err(_) => redirec_to_page(&format!("create?title={}", title)).into_response()
    }
}

pub async fn edit_doc_page(
    Path(title): Path<String>,
    AuthUser(username): AuthUser
) -> impl IntoResponse {
    match load_doc(&title) {
        Ok(md_content) => {
            let meta = load_doc_meta(&title).unwrap_or_default();
            Html(render_editor_html(&title, &md_content, &meta.tags, &username)).into_response()
        }
        Err(_) => redirec_to_page(&(format!("create?title={}", title))).into_response()
    }
}

pub async fn create_doc_page(
    Query(params): Query<HashMap<String, String>>,
    AuthUser(username): AuthUser,
) -> Html<String> {
    let title = params.get("title").cloned().unwrap_or_default();

    Html(render_create_html(&title, &username))
}