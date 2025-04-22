use crate::storage::*;

pub fn render_viewer_html(name: &str, html: &str, tags: &[String], history: &[EditLog]) -> String {
    let tag_links = render_tags(tags);
    let history_html = render_history_section(history);
    let template = load_template_file("viewer.html").unwrap_or_default();

    template
        .replace("{title}", name)
        .replace("{tags}", &tag_links)
        .replace("{escaped_html}", &serde_json::to_string(html).unwrap())
        .replace("{history_section}", &history_html)
}

pub fn render_editor_html(name: &str, html: &str) -> String {
    let template = load_template_file("editor.html").unwrap_or_default();

    template
        .replace("{name}", name)
        .replace("{html}", html)
}

fn render_tags(tags: &[String]) -> String {
    tags.iter()
        .map(|tag| format!(r#"<a href="/tag/{}" class="tag">#{}</a>"#, tag, tag))
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