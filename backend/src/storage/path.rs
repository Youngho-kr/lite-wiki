use std::path::PathBuf;
use crate::config::{DOCS_PATH, UPLOADS_PATH};

pub fn doc_path(name: &str) -> PathBuf {
    docs_dir_path().join(format!("{name}.md"))
}

pub fn doc_meta_path(name: &str) -> PathBuf {
    docs_dir_path().join(format!("{name}.meta.json"))
}

pub fn docs_dir_path() -> PathBuf {
    DOCS_PATH.clone().into()
}

pub fn uploads_path(name: &str) -> PathBuf {
    uploads_dir_path().join(name)
}

pub fn uploads_dir_path() -> PathBuf {
    UPLOADS_PATH.clone().into()
}