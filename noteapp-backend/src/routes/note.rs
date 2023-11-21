use axum::{Json, Extension};
use http::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;

use crate::models::note::Note;

use super::error::ApiError;

#[derive(Deserialize, Debug)]
pub(super) struct NewNote {
    label: Option<String>,
    contents: String,
}


pub(super) async fn create_note(
    Extension(pool): Extension<PgPool>,
    Json(new_note): Json<NewNote>
) -> Result<(StatusCode, Json<Note>), ApiError> {



    // Ok((StatusCode::CREATED, ))

    todo!()
}
