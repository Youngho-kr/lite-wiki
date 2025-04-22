use std::{fs, io};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct DocMeta {
    pub tags: Vec<String>,
    // pub created: Option<String>,
    // pub updated: Option<String>,
    // pub history: Vec<EditLog>,
}
// #[derive(Serialize, Deserialize)]
// pub struct EditLog {
//     pub timestamp: String,
//     pub editor: String,
//     pub summary: String,
// }

const DOC_DIR: &str = "./data/docs";

pub fn list_doc_names() -> std::io::Result<Vec<String>> {
    fs::create_dir_all(DOC_DIR)?;
    let entries = fs::read_dir(DOC_DIR)?
        .filter_map(|e| e.ok())
        .filter_map(|e| e.path().file_stem()?.to_str().map(String::from))
        .collect();
    Ok(entries)
}

pub fn load_doc(name: &str) -> std::io::Result<String> {
    let path = doc_path(name);
    let content = fs::read_to_string(path)?;

    let meta_path = doc_meta_path(name);
    if !meta_path.exists() {
        let meta = DocMeta::default();
        save_doc_meta(name, &meta)?;
    }

    Ok(content)
}

pub fn save_doc_content(name: &str, content: &str) -> std::io::Result<()> {
    let path = doc_path(name);
    fs::write(&path, content)?;

    let meta_path = doc_meta_path(name);
    if !meta_path.exists() {
        let meta = DocMeta::default();
        let meta_str = serde_json::to_string_pretty(&meta)?;
        fs::write(meta_path, meta_str)?;
    }

    Ok(())
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
        let meta = DocMeta::default();
        save_doc_meta(name, &meta)?;
        Ok(DocMeta::default())
    }
}

pub fn save_doc_meta(name: &str, meta: &DocMeta) -> io::Result<()> {
    let path = doc_meta_path(name);
    let content = serde_json::to_string_pretty(meta)?;
    fs::write(path, content)
}

fn doc_meta_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from("data/docs");
    path.push(format!("{}.meta.json", name));
    path
}