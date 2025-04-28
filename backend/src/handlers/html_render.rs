use crate::storage::*;

pub fn render_viewer_html(name: &str, html: &str, tags: &[String], history: &[EditLog]) -> String {
    let tag_links = render_viewer_tags(tags);
    let history_html = render_history_section(history);
    let template = load_template_file("viewer.html").unwrap_or_default();

    template
        .replace("{title}", name)
        .replace("{tags}", &tag_links)
        .replace("{escaped_html}", &serde_json::to_string(html).unwrap())
        .replace("{history_section}", &history_html)
}

pub fn render_editor_html(name: &str, html: &str, tags: &[String]) -> String {
    let template = load_template_file("editor.html").unwrap_or_default();
    let tag_links = render_editor_tags(tags);

    template
        .replace("{name}", name)
        .replace("{html}", html)
        .replace("{tags}", &tag_links)
}

pub fn render_search_result_html(keyword: &str, results: &[String]) -> String {
    let items: String = results
        .iter()
        .map(|name| format!(r#"<li><a href="/{}">{}</a></li>"#, name, name))
        .collect();

    let template = load_template_file("search_result.html").unwrap_or_default();

    template
        .replace("{keyword}", keyword)
        .replace("{results}", &items)
}

pub fn render_search_empty_html(keyword: &str) -> String {
    let template = load_template_file("search_empty.html").unwrap_or_default();

    template
        .replace("{keyword}", keyword)
}

pub fn render_search_no_input_html() -> String {
    let template = load_template_file("search_no_input.html").unwrap_or_default();

    template
}

pub fn render_template_list_html(template_names: &[String]) -> String {
    let items = template_names
        .iter()
        .map(|name| {
            let link = format!("/create?template={}", name);
            format!(r#"<li>{} — <a href="{}">이 템플릿으로 문서 만들기</a></li>"#, name, link)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let template = load_template_file("template_list.html").unwrap_or_default();

    template.replace("{items}", &items)
}

pub fn render_create_html(title: &str, content: &str) -> String {
    let template = load_template_file("create.html").unwrap_or_default();

    template
        .replace("{title}", title)
        .replace("{html}", &serde_json::to_string(content).unwrap())
}

pub fn render_doc_list_html(doc_names: &mut [String]) -> String {
    doc_names.sort();
    let items = doc_names
        .iter()
        .map(|name| format!(r#"<li><a href="/{}">{}</a></li>"#, name, name))
        .collect::<Vec<_>>()
        .join("\n");

    let template = load_template_file("doc_list.html").unwrap_or_default();

    template.replace("{items}", &items)
}

pub fn render_search_tag_html(tag: &str, docs: &mut [String]) -> String {
    docs.sort();
    let items = docs.iter()
        .map(|doc| format!(r#"<li><a href="/{}">{}</a></li>"#, doc, doc))
        .collect::<Vec<_>>()
        .join("\n");

    let template = load_template_file("search_tag.html").unwrap_or_default();

    template
        .replace("{tag}", tag)
        .replace("{items}", &items)
}

pub fn render_all_tags_html(tags: &[String]) -> String {
    let items = tags.iter()
        .map(|tag| format!(r#"<li><a href="/tags/{}" class="tag">#{}</a></li>"#, tag, tag))
        .collect::<Vec<_>>()
        .join("\n");

    let template = load_template_file("tag_list.html").unwrap_or_default();
    template.replace("{items}", &items)
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