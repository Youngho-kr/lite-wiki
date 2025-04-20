use std::fs;
use std::path::PathBuf;

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
    fs::read_to_string(doc_path(name))
}

pub fn save_doc_content(name: &str, content: &str) -> std::io::Result<()> {
    fs::create_dir_all(DOC_DIR)?;
    fs::write(doc_path(name), content)
}

pub fn delete_doc_file(name: &str) -> std::io::Result<()> {
    fs::remove_file(doc_path(name))
}

fn doc_path(name: &str) -> PathBuf {
    PathBuf::from(format!("{}/{}.md", DOC_DIR, name))
}