use axum::response::Html;
use crate::storage::list_doc_names;

use super::render_doc_list_html;

pub async fn render_doc_list() -> Html<String> {
    match list_doc_names() {
        Ok(mut names) => Html(render_doc_list_html(&mut names, "user")),
        Err(_) => Html("<h1>문서 목록을 불러올 수 없습니다.</h1>".to_string()),
    }
}