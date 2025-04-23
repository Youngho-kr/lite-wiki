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
    pub fn new(summary: &str) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            tags: Vec::new(),
            created: Some(now.clone()),
            updated: Some(now.clone()),
            history: vec![EditLog {
                timestamp: now,
                editor: "anonymous".to_string(),
                summary: summary.to_string(),
                diff_summary: None
            }]
        }
    }

    pub fn record_edit(&mut self, summary: &str, before: Option<&str>, after: Option<&str>) {
        let now = Utc::now().to_rfc3339();
        if self.created.is_none() {
            self.created = Some(now.clone());
        }
        self. updated = Some(now.clone());

        let diff_summary = match (before, after) {
            (Some(b), Some(a)) if b != a => Some(generate_diff(b, a)),
            _ => None,
        };

        self.history.push(EditLog { 
            timestamp: now, 
            editor: "annonymous".to_string(), 
            summary: summary.to_string(),
            diff_summary: diff_summary,
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
        let meta = DocMeta::new("create");
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