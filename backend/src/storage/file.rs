use tokio::fs;
use regex::Regex;
use uuid::Uuid;
use crate::storage::uploads_dir_path;

pub const ALLOWED_EXTENSIONS: [&str; 5] = ["png", "jpg", "jpeg", "webp", "gif"];
pub fn sanitize_filename(raw: &str) -> String {
    let filename_regex = Regex::new(r"[^\w\d_-]").unwrap();
    filename_regex.replace_all(raw, "_").to_string()
}

pub fn generate_filename(provided_name: Option<&str>, extension: &str) -> String {
    let base_name = match provided_name {
        Some(name) if !name.trim().is_empty() => sanitize_filename(name.trim()),
        _ => Uuid::new_v4().to_string(),
    };
    format!("{}.{}", base_name, extension)
}

pub async fn save_file(final_filename: &str, data: &[u8]) -> Result<(), std::io::Error> {
    let filepath = uploads_dir_path().join(final_filename);

    if filepath.exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "File already exists"));
    }

    fs::write(&filepath, data).await
}