use axum::{
    extract::{Path, Json}, 
    response::IntoResponse, 
    http::StatusCode,
};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use crate::{auth::AuthUser, storage::*};

#[derive(Deserialize)]
pub struct SaveDoc {
    pub content: String,
    pub tags: Vec<String>
}

#[derive(Serialize)]
pub struct Doc {
    pub name: String,
    pub content: String,
}

pub async fn list_docs() -> impl IntoResponse {
    match list_doc_names() {
        Ok(names) => Json(names).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read").into_response(),
    }
}

pub async fn get_doc(Path(name): Path<String>) ->  impl IntoResponse {
    match load_doc(&name) {
        Ok(content) => Json(Doc { name, content }).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Not found").into_response(),
    }
}

#[debug_handler]
pub async fn post_doc(
    Path(name): Path<String>, 
    AuthUser(username): AuthUser,
    Json(payload): Json<SaveDoc> 
) -> impl IntoResponse {
    match save_doc(&name, &payload.content, &payload.tags, &username) {
        Ok(_) => (StatusCode::OK, "Saved").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save").into_response(),
    }
}

pub async fn delete_doc(Path(name): Path<String>) -> impl IntoResponse {
    match delete_doc_file(&name) {
        Ok(_) => (StatusCode::OK, "Deleted").into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Not found").into_response(),
    }
}

pub async fn check_doc_exists(Path(name): Path<String>) -> impl IntoResponse {
    match load_doc(&name) {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(_) => (StatusCode::NO_CONTENT).into_response(),
    }
}
#[derive(Serialize, Deserialize)]
pub struct TagUpdateRequest {
    tags: Vec<String>,
}

#[derive(Serialize)]
pub struct TagListResponse {
    name: String,
    tags: Vec<String>,
}

pub async fn get_tags(Path(name): Path<String>) -> impl IntoResponse {
    match load_doc_meta(&name) {
        Ok(meta) => Json(TagListResponse { name, tags: meta.tags }).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "No metadata").into_response(),
    }
}

// pub async fn update_tags(
//     Path(name): Path<String>,
//     AuthUser(username): AuthUser,
//     Json(payload): Json<TagUpdateRequest>
// ) -> impl IntoResponse {
//     let mut meta = load_doc_meta(&name).unwrap_or_default();
//     // Update tags
//     meta.tags = payload.tags;

//     match save_doc_meta(&name, &meta) {
//         Ok(_) => (StatusCode::OK, "Success to save").into_response(),
//         Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save").into_response(),
//     }
// }