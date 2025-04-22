use axum::{
    extract::Path, 
    response::{Html, IntoResponse, Redirect}
};
use crate::storage::{load_doc, load_doc_meta};
use pulldown_cmark::{Parser, Options, html};

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
    let tag_links: String = tags
        .iter()
        .map(|tag| format!(r#"<a href="/tag/{}" class="tag">#{}</a>"#, tag, tag))
        .collect::<Vec<_>>()
        .join(" ");

    format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8" />
            <title>{name}</title>
            <link rel="stylesheet" href="https://uicdn.toast.com/editor/latest/toastui-editor.min.css" />
            <style>
                body {{ font-family: sans-serif; max-width: 720px; margin: auto; padding: 2rem; }}
                #topbar {{
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    margin-bottom: 1.5rem;
                    gap: 1rem;
                }}
                #edit-btn {{
                    background: #eee;
                    border: 1px solid #ccc;
                    padding: 4px 8px;
                    border-radius: 4px;
                    text-decoration: none;
                    color: #333;
                }}
                #edit-btn:hover {{ background: #ddd; }}
                #search-form input {{
                    padding: 4px;
                    border: 1px solid #ccc;
                    border-radius: 4px;
                }}
                #tags {{
                    margin-bottom: 1rem;
                }}
                .tag {{
                    display: inline-block;
                    background-color: #f3f3f3;
                    border-radius: 4px;
                    padding: 2px 6px;
                    margin-right: 6px;
                    font-size: 0.9em;
                    text-decoration: none;
                    color: #0366d6;
                }}
                .tag:hover {{
                    background-color: #e6e6e6;
                }}
            </style>
        </head>
        <body>
            <div id="topbar">
                <h1>{name}</h1>
                <a id="edit-btn" href="/edit/{name}">✏️ 수정</a>
            </div>

            <div id="tags">{tag_links}</div>

            <div id="viewer"></div>

            <script src="https://uicdn.toast.com/editor/latest/toastui-editor-all.min.js"></script>
            <script>
                const markdownContent = {escaped_json};
                toastui.Editor.factory({{
                    el: document.querySelector('#viewer'),
                    viewer: true,
                    initialValue: markdownContent,
                }});
            </script>
            </br>
            <form id="search-form" action="/search" method="get">
                <input type="text" name="q" placeholder="문서 검색..." />
            </form>
        </body>
        </html>
        "#,
        name = name,
        escaped_json = serde_json::to_string(html).unwrap(),
        tag_links = tag_links
    )
}

// Render HTML editor page
fn render_editor_page(name: &str, escaped_markdown: &str) -> String {
    format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8" />
            <title>Edit: {name}</title>
            <link rel="stylesheet" href="https://uicdn.toast.com/editor/latest/toastui-editor.min.css" />
            <style>
                body {{ font-family: sans-serif; max-width: 720px; margin: auto; padding: 2rem; }}
                #editor {{ margin-bottom: 1rem; }}
            </style>
        </head>
        <body>
            <h1>Editing: {name}</h1>
            <div id="editor"></div>
            <button id="saveBtn">💾 저장</button>

            <script src="https://uicdn.toast.com/editor/latest/toastui-editor-all.min.js"></script>
            <script>
                const markdownContent = {escaped_markdown};
                const pageName = "{name}";

                const editor = new toastui.Editor({{
                    el: document.querySelector('#editor'),
                    height: '500px',
                    initialEditType: 'markdown',
                    previewStyle: 'vertical',
                    initialValue: markdownContent,
                }});

                document.querySelector('#saveBtn').addEventListener('click', () => {{
                    fetch("/api/docs/" + pageName, {{
                        method: "POST",
                        headers: {{ "Content-Type": "application/json" }},
                        body: JSON.stringify({{ content: editor.getMarkdown() }}),
                    }}).then(() => {{
                        alert("저장 완료!");
                        location.href = "/" + pageName;
                    }});
                }});
            </script>
        </body>
        </html>
        "#
    )
}