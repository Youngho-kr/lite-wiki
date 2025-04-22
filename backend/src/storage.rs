use std::{fs, io};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use chrono::Utc;

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
            }]
        }
    }

    pub fn record_edit(&mut self, summary: &str) {
        let now = Utc::now().to_rfc3339();
        if self.created.is_none() {
            self.created = Some(now.clone());
        }
        self. updated = Some(now.clone());
        self.history.push(EditLog { 
            timestamp: now, 
            editor: "annonymous".to_string(), 
            summary: summary.to_string(), 
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct EditLog {
    pub timestamp: String,
    pub editor: String,
    pub summary: String,
}

const DOC_DIR: &str = "./data/docs";

pub fn list_doc_names() -> io::Result<Vec<String>> {
    fs::create_dir_all(DOC_DIR)?;
    let entries = fs::read_dir(DOC_DIR)?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.path().file_name()?.to_str()?.to_string();
            if name.ends_with(".md") {
                Some(name.trim_end_matches(".md").to_string())
            } else {
                None
            }
        })
        .collect();
    Ok(entries)
}

pub fn load_doc(name: &str) -> io::Result<String> {
    let content = fs::read_to_string(doc_path(name))?;

    load_doc_meta(name).ok();

    Ok(content)
}

pub fn save_doc_content(name: &str, content: &str) -> io::Result<()> {
    fs::write(doc_path(name), content)?;

    let mut meta = load_doc_meta(name).unwrap_or_default();
    meta.record_edit("save");

    save_doc_meta(name, &meta)
}

pub fn delete_doc_file(name: &str) -> std::io::Result<()> {
    fs::remove_file(doc_path(name))
}

fn doc_path(name: &str) -> PathBuf {
    PathBuf::from(format!("{}/{}.md", DOC_DIR, name))
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

fn doc_meta_path(name: &str) -> PathBuf {
    PathBuf::from(format!("{}/{}.meta.json", DOC_DIR, name))
}