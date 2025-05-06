use axum::{http::StatusCode, response::{IntoResponse, Redirect}};
use rand::seq::SliceRandom;

use crate::storage::list_doc_names;

pub async  fn random_page() -> impl IntoResponse {
    let Ok(entries) = list_doc_names() else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    if let Some(choice) = entries.choose(&mut rand::thread_rng()) {
        Ok(Redirect::to(&format!("/{}", &choice)))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}