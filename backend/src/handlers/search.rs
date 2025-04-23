use axum::{
    extract::Query, 
    response::{Html, IntoResponse},
};
use std::collections::HashMap;
use crate::storage::list_doc_names;
use crate::handlers::{render_search_result_html, render_search_empty_html, render_search_no_input_html};

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
                        render_search_empty_html(keyword)
                    } else {
                        // render_search_page(keyword, &matches)
                        render_search_result_html(keyword, &matches)
                    };

                    Html(html).into_response()
                }
                Err(_) => Html("<h1>검색 중 오류 발생</h1>".to_string()).into_response(),
            }
        }
        None => {
            Html(render_search_no_input_html()).into_response()
        }
    }
}
