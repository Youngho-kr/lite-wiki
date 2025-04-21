use axum::response::Html;
use crate::storage::list_doc_names;

pub async fn render_doc_list() -> Html<String> {
    match list_doc_names() {
        Ok(mut names) => {
            names.sort(); // 가나다 순 정렬

            let items: String = names
                .iter()
                .filter(|name| name != &"docs") // 자기 자신은 제외
                .map(|name| format!(r#"<li><a href="/{}">{}</a></li>"#, name, name))
                .collect();

            Html(format!(
                r#"
                <!DOCTYPE html>
                <html lang="ko">
                <head>
                    <meta charset="UTF-8">
                    <title>📚 전체 문서 목록</title>
                    <style>
                        body {{
                            font-family: sans-serif;
                            max-width: 720px;
                            margin: auto;
                            padding: 2rem;
                        }}
                        h1 {{
                            margin-bottom: 1rem;
                        }}
                        ul {{
                            list-style: none;
                            padding-left: 0;
                        }}
                        li {{
                            margin: 0.5rem 0;
                        }}
                        a {{
                            text-decoration: none;
                            color: #0366d6;
                        }}
                        a:hover {{
                            text-decoration: underline;
                        }}
                    </style>
                </head>
                <body>
                    <h1>📚 전체 문서 목록</h1>
                    <ul>
                        {items}
                    </ul>
                    <p><a href="/edit/새문서">➕ 새 문서 만들기</a></p>
                </body>
                </html>
                "#
            ))
        }
        Err(_) => Html("<h1>문서 목록을 불러올 수 없습니다.</h1>".to_string()),
    }
}