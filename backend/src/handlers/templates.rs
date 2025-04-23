use axum::response::Html;
use std::{fs, path::PathBuf};
use crate::handlers::render_template_list_html;

pub async fn render_template_list() -> Html<String> {
    let mut names = vec![];

    if let Ok(entries) = fs::read_dir("data/templates") {
        for entry in entries.flatten() {
            if let Some(name) = entry.path().file_stem().and_then(|s| s.to_str()) {
                names.push(name.to_string());
            }
        }
    }

    Html(render_template_list_html(&names))
}

pub fn load_template(name: &str) -> std::io::Result<String> {
    let mut path = PathBuf::from("data/templates");
    path.push(format!("{name}.md"));
    fs::read_to_string(path)
}