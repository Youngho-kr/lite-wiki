use axum::response::Html;
use std::fs;

pub async fn render_template_list() -> Html<String> {
    let mut items = String::new();

    if let Ok(entries) = fs::read_dir("data/templates") {
        for entry in entries.flatten() {
            if let Some(name) = entry.path().file_stem().and_then(|s| s.to_str()) {
                let link = format!("/edit/새문서?template={}", name);
                items.push_str(&format!(
                    r#"<li>{} — <a href="{}">이 템플릿으로 문서 만들기</a></li>"#,
                    name, link
                ));
            }
        }
    }

    Html(format!(
        r#"
        <!DOCTYPE html>
        <html lang="ko">
        <head>
            <meta charset="UTF-8">
            <title>🧩 템플릿 목록</title>
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
            <h1>🧩 템플릿 목록</h1>
            <ul>
                {items}
            </ul>
        </body>
        </html>
        "#,
        items = items
    ))
}