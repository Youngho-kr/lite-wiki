use axum::{
    extract::{Json, Path}, http::StatusCode, response::{Html, IntoResponse, Redirect}
};
use serde::{Deserialize, Serialize};
use crate::storage::*;

#[derive(Deserialize)]
pub struct SaveDoc {
    content: String,
}

#[derive(Serialize)]
pub struct Doc {
    name: String,
    content: String,
}

pub async fn list_docs() -> impl IntoResponse {
    match list_doc_names() {
        Ok(names) => Json(names).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read").into_response(),
    }
}

pub async fn get_doc(Path(name): Path<String>) ->  impl IntoResponse {
    match load_doc(&name) {
        Ok(content) => Json(Doc { name, content }).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Not found").into_response(),
    }
}

pub async fn save_doc(Path(name): Path<String>, Json(payload): Json<SaveDoc>) -> impl IntoResponse {
    match save_doc_content(&name, &payload.content) {
        Ok(_) => (StatusCode::OK, "Saved").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save").into_response(),
    }
}

pub async fn delete_doc(Path(name): Path<String>) -> impl IntoResponse {
    match delete_doc_file(&name) {
        Ok(_) => (StatusCode::OK, "Deleted").into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Not found").into_response(),
    }
}
pub async fn render_doc_html(Path(name): Path<String>) -> Html<String> {
    match load_doc(&name) {
        Ok(content) => {
            let escaped = serde_json::to_string(&content).unwrap(); // JS에 넘기기 위해 escape

            Html(format!(
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
                        }}
                        #topbar h1 {{
                            margin: 0;
                        }}
                        #edit-btn {{
                            background: #eee;
                            border: 1px solid #ccc;
                            padding: 4px 8px;
                            border-radius: 4px;
                            text-decoration: none;
                            color: #333;
                        }}
                        #edit-btn:hover {{
                            background: #ddd;
                        }}
                    </style>
                </head>
                <body>
                    <div id="topbar">
                        <h1>{name}</h1>
                        <a id="edit-btn" href="/edit/{name}">✏️ 수정</a>
                    </div>

                    <div id="viewer"></div>

                    <script src="https://uicdn.toast.com/editor/latest/toastui-editor-all.min.js"></script>
                    <script>
                        const markdownContent = {escaped};
                        toastui.Editor.factory({{
                            el: document.querySelector('#viewer'),
                            viewer: true,
                            initialValue: markdownContent,
                        }});
                    </script>
                </body>
                </html>
                "#,
                name = name,
                escaped = escaped
            ))
        }
        Err(_) => Html("<h1>404 Not Found</h1>".to_string()),
    }
}

pub async fn edit_doc_page(Path(name): Path<String>) -> Html<String> {
    let content = load_doc(&name).unwrap_or_default(); // 없으면 빈 문서
    let escaped = serde_json::to_string(&content).unwrap(); // JS에서 안전하게 쓸 수 있도록 escape

    Html(format!(
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
                const markdownContent = {escaped};
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
        "#,
        name = name,
        escaped = escaped
    ))
}

pub async fn redirect_to_index() -> Redirect {
    Redirect::temporary("/index")
}
