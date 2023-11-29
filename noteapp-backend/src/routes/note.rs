use axum::{Json, Extension};
use http::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::note::Note;
use super::error::ApiError;

#[derive(Deserialize, Debug)]
pub(super) struct NewNote {
    pub label: Option<String>,
    pub contents: String,
}

pub(super) async fn create_note(
    Extension(pool): Extension<PgPool>,
    Json(new_note): Json<NewNote>
) -> Result<(StatusCode, Json<Note>), ApiError> {

    let note = Note::from(new_note)
        .insert(&pool).await?;

    Ok((StatusCode::CREATED, Json(note)))
}

impl From<NewNote> for Note {
    fn from(value: NewNote) -> Self {
        Note {
            id: Uuid::new_v4(),
            label: value.label,
            contents: value.contents
        }
    }
}
