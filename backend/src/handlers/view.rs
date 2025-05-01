use std::collections::HashMap;

use axum::{
    extract::{Path, Query}, 
    response::{Html, IntoResponse, Redirect}
};
use crate::handlers::html_render::*;
use crate::storage::{load_doc, load_doc_meta, load_template};

pub async fn render_doc_page(Path(name): Path<String>) -> impl IntoResponse {
    match load_doc(&name) {
        Ok(md_content) => {
            let meta = load_doc_meta(&name).unwrap_or_default();
            Html(render_viewer_html(&name, &md_content, &meta.tags, &meta.history, "user")).into_response()
        }
        Err(_) => Redirect::to(&format!("/create?title={}", name)).into_response(),
    }
}

pub async fn edit_doc_page(Path(name): Path<String>,) -> impl IntoResponse {
    match load_doc(&name) {
        Ok(md_content) => {
            let meta = load_doc_meta(&name).unwrap_or_default();
            Html(render_editor_html(&name, &md_content, &meta.tags, "user")).into_response()
        }
        Err(_) => Redirect::to(&format!("/create?title={}", name)).into_response(),
    }
}

pub async fn create_doc_page(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let title = params.get("title").cloned().unwrap_or_default();

    let content = params
        .get("template")
        .and_then(|tpl| load_template(tpl).ok())
        .unwrap_or_default();

    Html(render_create_html(&title, &content, "user"))
}