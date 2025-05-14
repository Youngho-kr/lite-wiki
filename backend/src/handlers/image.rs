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

use crate::storage::{file, ALLOWED_EXTENSIONS};

#[derive(Deserialize)]
pub struct UploadParams {
    filename: Option<String>,
}

pub async fn upload_image(
    Query(params): Query<UploadParams>,
    mut multipart: Multipart
) -> Result<String, StatusCode> {
    let field = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)?
        .ok_or(StatusCode::BAD_REQUEST)?;

    let original_filename = field.file_name().unwrap_or("file");
    let extension = std::path::Path::new(original_filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .ok_or(StatusCode::UNSUPPORTED_MEDIA_TYPE)?;

    if !ALLOWED_EXTENSIONS.contains(&&extension.as_str()) {
        return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    let safe_filename = file::generate_filename(params.filename.as_deref(), &extension);

    let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
    file::save_file(&safe_filename, &data)
        .await
        .map_err(|err| if err.kind() == std::io::ErrorKind::AlreadyExists {
            StatusCode::CONFLICT
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(format!("/api/images/{}", safe_filename))
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