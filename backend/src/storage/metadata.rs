use chrono::Utc;
use serde::{Serialize, Deserialize};
use similar::{ChangeTag, TextDiff};
use std::{io, fs};

use crate::storage::*;

#[derive(Serialize, Deserialize, Default)]
pub struct DocMeta {
    pub tags: Vec<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub history: Vec<EditLog>,
}

impl DocMeta {
    pub fn new(new_tags: &[String], editor: &str) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            tags: new_tags.to_vec(),
            created: Some(now.clone()),
            updated: Some(now.clone()),
            history: vec![EditLog {
                timestamp: now,
                editor: editor.to_string(),
                summary: "create".to_string(),
                diff_summary: None
            }]
        }
    }

    pub fn record_edit(&mut self, editor: &str, before: Option<&str>, after: Option<&str>, tags: &[String]) {
        let now = Utc::now().to_rfc3339();
        if self.created.is_none() {
            self.created = Some(now.clone());
        }
        self.updated = Some(now.clone());

        let diff_summary = match (before, after) {
            (Some(b), Some(a)) if b != a => Some(generate_diff(b, a)),
            _ => None,
        };

        let mut summary = Vec::new();
        if diff_summary != None {
            summary.push("edit");
        }
        if self.tags != tags {
            println!("tag changed");
            summary.push("tag changed");
            self.tags = tags.to_vec();
        }

        self.history.push(EditLog { 
            timestamp: now, 
            editor: editor.to_string(), 
            summary: summary.join(", "),
            diff_summary,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct EditLog {
    pub timestamp: String,
    pub editor: String,
    pub summary: String,
    pub diff_summary: Option<String>,
}

pub fn load_doc_meta(name: &str) -> std::io::Result<DocMeta> {
    let path = doc_meta_path(name);
    if path.exists() {
        let content = fs::read_to_string(&path)?;
        let meta = serde_json::from_str(&content)?;
        Ok(meta)
    } else {
        let meta = DocMeta::new(&[],"annonymous");
        save_doc_meta(name, &meta)?;
        Ok(meta)
    }
}

pub fn save_doc_meta(name: &str, meta: &DocMeta) -> io::Result<()> {
    let path = doc_meta_path(name);
    let content = serde_json::to_string_pretty(meta)?;
    fs::write(path, content)
}

pub fn generate_diff(before: &str, after: &str) -> String {
    let diff = TextDiff::from_lines(before, after);

    let mut result = String::new();
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "- ",
            ChangeTag::Insert => "+ ",
            ChangeTag::Equal => "  ",
        };
        result.push_str(&format!("{}{}", sign, change));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{setup_test_env, clear_test_doc};

    #[test]
    fn test_docmeta_new() {
        let meta = DocMeta::new(&[],"test_editor");

        assert!(meta.created.is_some());
        assert!(meta.updated.is_some());
        assert_eq!(meta.created, meta.updated);
        assert_eq!(meta.history.len(), 1);
        assert_eq!(meta.history[0].summary, "create");
    }

    #[test]
    fn test_save_and_load_meta() {
        setup_test_env();

        let title = "test_save_and_load_meta";
        clear_test_doc(title);

        let editor = "test_editor";

        let mut meta = DocMeta::new(&[], editor);
        meta.tags.push("test".to_string());

        save_doc_meta(title, &meta).unwrap();
        let loaded = load_doc_meta(title).unwrap();

        assert_eq!(loaded.tags, vec!["test"]);
        assert_eq!(loaded.history.len(), 1);
    }

    #[test]
    fn test_generate_diff() {
        let before = "Hello\nWorld";
        let after = "Hello\nLiteWiki";
        let diff = generate_diff(before, after);

        assert!(diff.contains("- World"));
        assert!((diff.contains("+ LiteWiki")));
    }

    #[test]
    fn test_record_edit() {
        let mut meta = DocMeta::new(&[], "test_editor");
        let before = "Old content";
        let after = "New content";

        meta.record_edit("test_editor", Some(before), Some(after), &[]);

        assert_eq!(meta.history.len(), 2);

        let last = meta.history.last().unwrap();
        assert_eq!(last.summary, "edit");
    } 
}