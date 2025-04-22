use axum::{
    extract::Path, 
    response::{Html, IntoResponse, Redirect}
};
use crate::storage::{load_doc, load_doc_meta};
use pulldown_cmark::{Parser, Options, html};
use std::{io, fs};

pub async fn render_doc_html(Path(name): Path<String>) -> impl IntoResponse {
    match load_doc(&name) {
        Ok(md_content) => {
            let meta = load_doc_meta(&name).unwrap_or_default();
            let html_output = markdown_to_html(&md_content);
            Html(render_viewer_page(&name, &html_output, &meta.tags)).into_response()
        }
        Err(_) => Redirect::to(&format!("/edit/{}", name)).into_response(),
    }
}

pub async fn edit_doc_page(Path(name): Path<String>) -> Html<String> {
    let content = load_doc(&name).unwrap_or_default(); // 없으면 빈 문서
    let escaped = serde_json::to_string(&content).unwrap(); // JS에서 안전하게 쓸 수 있도록 escape
    Html(render_editor_page(&name, &escaped))
}

fn markdown_to_html(md: &str) -> String {
    let parser = Parser::new_ext(md, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

// Render HTML view page
// Viewer page contains search and edit button
fn render_viewer_page(name: &str, html: &str, tags: &[String]) -> String {
    let tag_links = tags
        .iter()
        .map(|tag| format!(r#"<a href="/tag/{}" class="tag">#{}</a>"#, tag, tag))
        .collect::<Vec<_>>()
        .join(" ");

    let template = load_template_file("viewer.html")
        .unwrap_or_else(|_| "<h1>Template not found</h1>".to_string());

    template
        .replace("{title}", name)
        .replace("{tags}", &tag_links)
        .replace("{escaped_html}", &serde_json::to_string(html).unwrap())
}

// Render HTML editor page
fn render_editor_page(name: &str, escaped_markdown: &str) -> String {
    let template = load_template_file("editor.html")
        .unwrap_or_else(|_| "<h1>Editor template not found</h1>".to_string());

    template
        .replace("{name}", name)
        .replace("{escaped_markdown}", escaped_markdown)
}

fn load_template_file(name: &str) -> io::Result<String> {
    let path = format!("static/html/{}", name);
    fs::read_to_string(path)
}