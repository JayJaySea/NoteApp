use axum::{Json, Extension, extract::Query};
use axum_extra::extract::CookieJar;
use http::{StatusCode,  HeaderMap};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::{models::note::Note, auth::{extract_token, decode_token}};
use super::{error::ApiError, AnyId};

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct NewNote {
    pub label: Option<String>,
    pub contents: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct UpdateNote {
    pub id: Uuid,
    pub label: Option<String>,
    pub contents: String,
}

pub(super) async fn create(
    cookie_jar: CookieJar,
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    Json(new_note): Json<NewNote>
) -> Result<(StatusCode, Json<Note>), ApiError> {
    let token = extract_token(cookie_jar, &headers)?;
    let claims = decode_token(&token)?;

    let note = Note::from_new(new_note, claims.sub)
        .insert(&pool).await?;

    Ok((StatusCode::CREATED, Json(note)))
}

pub(super) async fn read_all(
    cookie_jar: CookieJar,
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>
) -> Result<(StatusCode, Json<Vec<Note>>), ApiError> {
    let token = extract_token(cookie_jar, &headers)?;
    let claims = decode_token(&token)?;

    let notes = Note::select_user_notes(claims.sub, &pool).await?;

    Ok((StatusCode::OK, Json(notes)))
}

pub(super) async fn update(
    cookie_jar: CookieJar,
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    Json(note): Json<UpdateNote>
) -> Result<StatusCode, ApiError> {
    let token = extract_token(cookie_jar, &headers)?;
    let claims = decode_token(&token)?;

    Note::from_update(note, claims.sub)
        .update(&pool).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub(super) async fn delete(
    cookie_jar: CookieJar,
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    Query(AnyId{id}): Query<AnyId>,
) -> Result<StatusCode, ApiError> {
    let token = extract_token(cookie_jar, &headers)?;
    let claims = decode_token(&token)?;

    Note::delete(id, claims.sub, &pool).await?;

    Ok(StatusCode::NO_CONTENT)
}

impl Note {
    fn from_new(value: NewNote, user_id: Uuid) -> Self {
        Note {
            id: Uuid::new_v4(),
            label: value.label,
            contents: value.contents,
            user_id
        }
    }
    fn from_update(value: UpdateNote, user_id: Uuid) -> Self {
        Note {
            id: value.id,
            label: value.label,
            contents: value.contents,
            user_id
        }
    }
}
