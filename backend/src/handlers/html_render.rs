use crate::storage::*;
use std::collections::HashMap;

fn render_template(template_name: &str, vars: &HashMap<&str, String>) -> String {
    let content = load_template_file(template_name).unwrap_or_default();
    let mut result = content;

    for (key, value) in vars {
        result = result.replace(&format!("{{{}}}", key), value)
    }

    result
}

fn render_layout(content: &str, username: &str) -> String {
    let mut vars = HashMap::new();

    vars.insert("content", content.to_string());
    vars.insert("username", username.to_string());

    render_template("layout.html", &vars)
}

fn render_full_page(template_name: &str, vars: &HashMap<&str, String>, username: &str) -> String {
    let content = render_template(template_name, vars);
    render_layout(&content, username)
}

pub fn render_viewer_html(
    title: &str, 
    html: &str, 
    tags: &[String],
    history: &[EditLog],
    username: &str,
) -> String {
    let mut vars = HashMap::new();

    vars.insert("title", title.to_string());
    vars.insert("md", serde_json::to_string(html).unwrap());
    vars.insert("tags", render_viewer_tags(tags));
    vars.insert("history_section", render_history_section(history));

    render_full_page("viewer.html", &vars, username)
}

pub fn render_editor_html(
    title: &str, 
    html: &str, 
    tags: &[String],
    username: &str
) -> String {
    let mut vars = HashMap::new();

    vars.insert("title", title.to_string());
    vars.insert("md", serde_json::to_string(html).unwrap());
    vars.insert("tags", render_editor_tags(tags));

    render_full_page("editor.html", &vars, username)
}

pub fn render_search_result_html(
    keyword: &str, 
    results: &[String],
    username: &str
) -> String {
    let items: String = results
    .iter()
    .map(|name| format!(r#"<li><a href="/{}">{}</a></li>"#, name, name))
    .collect();

    let mut vars = HashMap::new();

    vars.insert("keyword", keyword.to_string());
    vars.insert("results", items.to_string());

    render_full_page("search_result.html", &vars, username)
}

pub fn render_search_empty_html(
    keyword: &str,
    username: &str
) -> String {
    let mut vars = HashMap::new();

    vars.insert("keyword", keyword.to_string());

    render_full_page("search_empty.html", &vars, username)
}

pub fn render_search_no_input_html(
    username: &str
) -> String {
    render_full_page("search_no_input.html", &mut HashMap::new(), username)
}

pub fn render_template_list_html(
    template_names: &[String],
    username: &str
) -> String {
    let items = template_names
    .iter()
    .map(|name| {
        let link = format!("/create?template={}", name);
        format!(r#"<li>{} — <a href="{}">이 템플릿으로 문서 만들기</a></li>"#, name, link)
    })
    .collect::<Vec<_>>()
    .join("\n");

    let mut vars = HashMap::new();

    vars.insert("items", items.to_string());

    render_full_page("template_list.html", &vars, username)
}

pub fn render_create_html(
    title: &str, 
    content: &str,
    username: &str,
) -> String {
    let mut vars = HashMap::new();

    vars.insert("title", title.to_string());
    vars.insert("html",serde_json::to_string(content).unwrap());

    render_full_page("create.html", &vars, username)
}

pub fn render_doc_list_html(
    doc_names: &mut [String],
    username: &str,
) -> String {
    doc_names.sort();
    let items = doc_names
    .iter()
    .map(|name| format!(r#"<li><a href="/{}">{}</a></li>"#, name, name))
    .collect::<Vec<_>>()
    .join("\n");

    let mut vars = HashMap::new();

    vars.insert("items", items.to_string());

    render_full_page("doc_list.html", &vars, username)
}

pub fn render_search_tag_html(
    tag: &str, 
    docs: &mut [String],
    username: &str,
) -> String {
    docs.sort();
    let items = docs.iter()
        .map(|doc| format!(r#"<li><a href="/{}">{}</a></li>"#, doc, doc))
        .collect::<Vec<_>>()
        .join("\n");

    let mut vars = HashMap::new();

    vars.insert("tag", tag.to_string());
    vars.insert("items", items.to_string());

    render_full_page("search_tag.html", &vars, username)
}

pub fn render_all_tags_html(
    tags: &[String],
    username: &str,
) -> String {
    let items = tags.iter()
        .map(|tag| format!(r#"<li><a href="/tags/{}" class="tag">#{}</a></li>"#, tag, tag))
        .collect::<Vec<_>>()
        .join("\n");

    let mut vars = HashMap::new();

    vars.insert("items", items.to_string());

    render_full_page("tag_list.html", &vars, username)
}

pub fn render_login_page_html() -> String {
    let template = load_template_file("login.html").unwrap_or_default();

    template
}

pub fn render_signup_page_html() -> String {
    let template = load_template_file("signup.html").unwrap_or_default();

    template
}

fn render_viewer_tags(tags: &[String]) -> String {
    tags.iter()
        .map(|tag| format!(r#"<a href="/tag/{}" class="tag">#{}</a>"#, tag, tag))
        .collect::<Vec<_>>()
        .join(" ")
}

fn render_editor_tags(tags: &[String]) -> String {
    tags.iter()
        .map(|tag| format!(r#"<span class="tag">#{tag}<span class="remove-tag">×</span></span>"#))
        .collect::<Vec<_>>()
        .join(" ")
}

fn render_history_section(history: &[EditLog]) -> String {
    if history.is_empty() {
        return "".to_string();
    }

    let mut logs = String::new();
    for log in history.iter().rev() {
        let diff = log.diff_summary.as_deref().unwrap_or("(no diff)");
        logs.push_str(&format!(
            r#"<div class="log-entry">
                <div class="log-header"><strong>{}</strong> — {}<br /><em>{}</em></div>
                <pre class="log-diff">{}</pre>
            </div>"#,
            log.timestamp,
            log.editor,
            log.summary,
            html_escape::encode_text(diff)
        ));
    }

    let section = load_template_file("history_section.html").unwrap_or_default();
    section.replace("{logs}", &logs)
}

fn load_template_file(name: &str) -> std::io::Result<String> {
    std::fs::read_to_string(format!("static/html/{}", name))
}