use axum::{Json, Extension, extract::Query};
use http::StatusCode;
use serde::Deserialize;
use tracing::debug;
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::note::Note;
use super::{error::ApiError, AnyId};

#[derive(Deserialize, Debug)]
pub(super) struct NewNote {
    pub label: Option<String>,
    pub contents: String,
}

pub(super) async fn create(
    Extension(pool): Extension<PgPool>,
    Json(new_note): Json<NewNote>
) -> Result<(StatusCode, Json<Note>), ApiError> {
    let note = Note::from(new_note)
        .insert(&pool).await?;

    Ok((StatusCode::CREATED, Json(note)))
}

pub(super) async fn read(
    Extension(pool): Extension<PgPool>,
    Query(AnyId{id}): Query<AnyId>
) -> Result<(StatusCode, Json<Note>), ApiError> {
    let note = Note::select(id, &pool).await?;

    Ok((StatusCode::OK, Json(note)))
}

pub(super) async fn update(
    Extension(pool): Extension<PgPool>,
    Json(note): Json<Note>
) -> Result<StatusCode, ApiError> {
    note.update(&pool).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub(super) async fn delete(
    Extension(pool): Extension<PgPool>,
    Query(AnyId{id}): Query<AnyId>
) -> Result<StatusCode, ApiError> {
    Note::delete(id, &pool).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub(super) async fn read_all(
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<Vec<Note>>), ApiError> {
    let notes = Note::select_all(&pool).await?;

    Ok((StatusCode::OK, Json(notes)))
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
