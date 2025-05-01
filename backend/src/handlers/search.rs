use axum::{
    extract::{Path, Query}, 
    response::{Html, IntoResponse},
};
use std::collections::HashMap;
use crate::storage::{find_docs_by_tag, list_all_tags, list_doc_names};
use crate::handlers::{render_search_result_html, render_search_empty_html, render_search_no_input_html};
use crate::handlers::{render_search_tag_html, render_all_tags_html};

pub async fn render_search_results(
    Query(params): Query<HashMap<String, String>>
) -> impl IntoResponse {
    let query = params.get("q").map(|s| s.trim()).filter(|s| !s.is_empty());

    match query {
        Some(keyword) => {
            match list_doc_names() {
                Ok(names) => {
                    let mut matches: Vec<_> = names
                        .into_iter()
                        .filter(|name| name.contains(keyword))
                        .collect();
                    matches.sort();

                    let html = if matches.is_empty() {
                        render_search_empty_html(keyword, "user")
                    } else {
                        // render_search_page(keyword, &matches)
                        render_search_result_html(keyword, &matches, "user")
                    };

                    Html(html).into_response()
                }
                Err(_) => Html("<h1>검색 중 오류 발생</h1>".to_string()).into_response(),
            }
        }
        None => {
            Html(render_search_no_input_html("user")).into_response()
        }
    }
}

pub async fn render_search_tags(Path(tag_name): Path<String>) -> impl IntoResponse {
    match find_docs_by_tag(&tag_name) {
        Ok(mut matched_docs) => Html(render_search_tag_html(&tag_name, &mut matched_docs, "user")).into_response(),
        Err(_) => Html("<h1>문서 목록 불러오기 실패</h1>".to_string()).into_response(),
    }
}

pub async fn render_all_tags() -> impl IntoResponse {
    let tags = list_all_tags().unwrap_or_default();
    Html(render_all_tags_html(&tags, "user"))
}
