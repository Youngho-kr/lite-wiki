use axum::{
    extract::{Path, Query},
    http::{header, StatusCode}, 
    response::{IntoResponse, Response}
};
use axum_extra::extract::Multipart;
use mime_guess::MimeGuess;
use regex::Regex;
use serde::Deserialize;
use tokio::fs;
use uuid::Uuid;
use std::path::Path as StdPath;

const ALLOWED_EXTENSION: [&str; 5] = ["png", "jpg", "jpeg", "webp", "gif"];
const UPLOAD_DIR: &str = "data/uploads/";

#[derive(Deserialize)]
pub struct UploadParams {
    filename: Option<String>,
}

pub async fn upload_image(
    Query(params): Query<UploadParams>,
    mut multipart: Multipart
) -> Result<String, StatusCode> {
    let filename_regex = Regex::new(r"[^\w\d_-]").unwrap();

    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let original_filename = field.file_name().unwrap_or("file");

        let extension = StdPath::new(original_filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
            .ok_or(StatusCode::UNSUPPORTED_MEDIA_TYPE)?;

        if !ALLOWED_EXTENSION.contains(&extension.as_str()) {
            return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE)
        }

        let base_filename = match params.filename {
            Some(ref name) if !name.trim().is_empty() => name.trim().to_string(),
            _ => Uuid::new_v4().to_string(),
        };

        let safe_filename = filename_regex.replace_all(&base_filename, "_");
        let final_filename = format!("{}.{}", safe_filename, extension);
        let filepath = format!("{}{}", UPLOAD_DIR, final_filename);

        if StdPath::new(&filepath).exists() {
            return Err(StatusCode::CONFLICT);
        }

        let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
        fs::write(&filepath, &data).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        return Ok(format!("/api/images/{}", final_filename));
    }

    Err(StatusCode::BAD_REQUEST)
}

pub async fn serve_image(Path(filename): Path<String>) -> impl IntoResponse {
    let filepath = format!("data/uploads/{}", filename);

    match fs::read(&filepath).await {
        Ok(file_bytes) => {
            // MIME 타입 추론 (확장자 기반)
            let mime_type = MimeGuess::from_path(&filepath).first_or_octet_stream();

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime_type.as_ref())
                .body(axum::body::Body::from(file_bytes))
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(axum::body::Body::from("File not found"))
            .unwrap(),
    }
}