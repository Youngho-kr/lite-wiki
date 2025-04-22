use axum::{
    extract::Path, 
    response::{Html, IntoResponse, Redirect}
};
use crate::handlers::html_render::*;
use crate::storage::{load_doc, load_doc_meta};

use pulldown_cmark::{Parser, Options, html};

pub async fn render_doc_html(Path(name): Path<String>) -> impl IntoResponse {
    match load_doc(&name) {
        Ok(md_content) => {
            let meta = load_doc_meta(&name).unwrap_or_default();
            let html_output = markdown_to_html(&md_content);
            Html(render_viewer_html(&name, &html_output, &meta.tags, &meta.history)).into_response()
        }
        Err(_) => Redirect::to(&format!("/edit/{}", name)).into_response(),
    }
}

pub async fn edit_doc_page(Path(name): Path<String>) -> Html<String> {
    let content = load_doc(&name).unwrap_or_default(); // 없으면 빈 문서
    let escaped = serde_json::to_string(&content).unwrap(); // JS에서 안전하게 쓸 수 있도록 escape
    Html(render_editor_html(&name, &escaped))
}

fn markdown_to_html(md: &str) -> String {
    let parser = Parser::new_ext(md, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}