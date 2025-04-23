use std::{fs, path::PathBuf, io};
use crate::config::TEMPLATE_PATH;

pub fn list_template_names() -> Vec<String> {
    let mut names = vec![];
    if let Ok(entries) = fs::read_dir(TEMPLATE_PATH.clone()) {
        for entry in entries.flatten() {
            if let Some(name) = entry.path().file_stem().and_then(|s| s.to_str()) {
                names.push(name.to_string());
            }
        }
    }
    names
}

pub fn load_template(name: &str) -> io::Result<String> {
    let mut path = PathBuf::from(TEMPLATE_PATH.clone());
    path.push(format!("{name}.md"));
    fs::read_to_string(path)
}
