use axum::{
    extract::Query, 
    response::{Html, IntoResponse},
};
use std::collections::HashMap;
use crate::storage::list_doc_names;

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
                        render_empty_search_page(keyword)
                    } else {
                        render_search_page(keyword, &matches)
                    };

                    Html(html).into_response()
                }
                Err(_) => Html("<h1>검색 중 오류 발생</h1>".to_string()).into_response(),
            }
        }
        None => {
            Html(render_search_prompt_page(true)).into_response()
        }
    }
}

fn render_search_page(keyword: &str, results: &[String]) -> String {
    let items: String = results
        .iter()
        .map(|name| format!(r#"<li><a href="/{}">{}</a></li>"#, name, name))
        .collect();

    format!(
        r#"
        <!DOCTYPE html>
        <html lang="ko">
        <head>
            <meta charset="UTF-8">
            <title>🔍 '{keyword}' 검색 결과</title>
            <style>
                body {{
                    font-family: sans-serif;
                    max-width: 720px;
                    margin: auto;
                    padding: 2rem;
                }}
                h1 {{ margin-bottom: 1rem; }}
                ul {{ list-style: none; padding-left: 0; }}
                li {{ margin: 0.5rem 0; }}
                a {{ text-decoration: none; color: #0366d6; }}
                a:hover {{ text-decoration: underline; }}
            </style>
        </head>
        <body>
            <h1>🔍 '{keyword}' 검색 결과</h1>
            <ul>{items}</ul>
            <p><a href="/docs">← 전체 문서 보기</a></p>
        </body>
        </html>
        "#,
        keyword = keyword,
        items = items
    )
}

fn render_empty_search_page(keyword: &str) -> String {
    format!(
        r#"
        <!DOCTYPE html>
        <html lang="ko">
        <head>
            <meta charset="UTF-8">
            <title>🔍 '{keyword}' 검색 결과 없음</title>
            <style>
                body {{
                    font-family: sans-serif;
                    max-width: 720px;
                    margin: auto;
                    padding: 2rem;
                    text-align: center;
                }}
                a {{
                    color: #0366d6;
                    text-decoration: none;
                }}
                a:hover {{
                    text-decoration: underline;
                }}
            </style>
        </head>
        <body>
            <h1>🔍 '{keyword}'에 대한 검색 결과가 없습니다</h1>
            <p><a href="/edit/{keyword}">➕ '{keyword}' 문서 만들기</a></p>
            <p><a href="/docs">← 전체 문서 목록 보기</a></p>
        </body>
        </html>
        "#,
        keyword = keyword
    )
}

fn render_search_prompt_page(with_notice: bool) -> String {
    let notice_html = if with_notice {
        r#"<div id="notice">⚠️ 검색어를 입력해주세요</div>"#
    } else {
        ""
    };

    format!(
        r#"
        <!DOCTYPE html>
        <html lang="ko">
        <head>
            <meta charset="UTF-8">
            <title>🔍 문서 검색</title>
            <style>
                body {{
                    font-family: sans-serif;
                    max-width: 720px;
                    margin: auto;
                    padding: 2rem;
                    text-align: center;
                }}
                input {{
                    padding: 6px 12px;
                    font-size: 1rem;
                    width: 60%;
                    max-width: 300px;
                }}
                button {{
                    padding: 6px 12px;
                    font-size: 1rem;
                    margin-left: 0.5rem;
                }}
                #notice {{
                    background: #ffe4e1;
                    color: #b00020;
                    padding: 0.75rem;
                    margin-bottom: 1rem;
                    border-radius: 5px;
                }}
            </style>
        </head>
        <body>
            {notice_html}
            <h1>🔍 문서 검색</h1>
            <form action="/search" method="get">
                <input type="text" name="q" placeholder="검색어 입력..." autofocus />
                <button type="submit">검색</button>
            </form>
            <p><a href="/docs">← 전체 문서 보기</a></p>

            <script>
                const notice = document.getElementById("notice");
                if (notice) {{
                    setTimeout(() => notice.remove(), 3000);
                }}
            </script>
        </body>
        </html>
        "#
    )
}