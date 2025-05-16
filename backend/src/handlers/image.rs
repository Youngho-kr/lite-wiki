use axum::{
    extract::{Path, Query},
    http::{header, StatusCode}, 
    response::{IntoResponse, Response}
};
use axum_extra::extract::Multipart;
use mime_guess::MimeGuess;
use serde::Deserialize;
use tokio::fs;

use crate::{storage::{file, uploads_path, ALLOWED_EXTENSIONS}};

#[derive(Deserialize)]
pub struct UploadParams {
    filename: Option<String>,
}

pub async fn upload_image(
    Query(params): Query<UploadParams>,
    mut multipart: Multipart
) -> Result<String, StatusCode> {
    println!("upload_image");

    let field = match multipart.next_field().await {
        Ok(Some(f)) => f,
        Ok(None) => return  Err(StatusCode::BAD_REQUEST),
        Err(_) => return Err(StatusCode::BAD_REQUEST)
    };
    
    let original_filename = field.file_name().unwrap_or("file");
    let extension = match std::path::Path::new(original_filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
    {
        Some(ext) => ext,
        None => return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE)
    };

    if !ALLOWED_EXTENSIONS.contains(&&extension.as_str()) {
        return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    let safe_filename = file::generate_filename(params.filename.as_deref(), &extension);

    let data = match field.bytes().await {
        Ok(bytes) => bytes,
        Err(_) => return Err(StatusCode::BAD_REQUEST)
    };
    
    if let Err(err) = file::save_file(&safe_filename, &data).await
    {
        if err.kind() == std::io::ErrorKind::AlreadyExists {
            return Err(StatusCode::CONFLICT);
        } else {
            println!("왜이래");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(format!("/images/{}", safe_filename))
}

pub async fn serve_image(Path(filename): Path<String>) -> impl IntoResponse {

    let filepath = uploads_path(&filename);

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