use std::path::PathBuf;
use crate::config::DATA_PATH;

pub fn doc_path(name: &str) -> PathBuf {
    docs_dir_path().join(format!("{name}.md"))
}

pub fn doc_meta_path(name: &str) -> PathBuf {
    docs_dir_path().join(format!("{name}.meta.json"))
}

pub fn docs_dir_path() -> PathBuf {
    DATA_PATH.clone().into()
}