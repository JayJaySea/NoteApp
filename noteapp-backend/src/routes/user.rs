use axum::{Json, Extension, extract::Query};
use http::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::user::{User, UserProfile};

use super::{error::ApiError, AnyId};

#[derive(Deserialize, Debug)]
pub(super) struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

pub(super) async fn create(
    Extension(pool): Extension<PgPool>,
    Json(new_user): Json<NewUser>
) -> Result<(StatusCode, Json<UserProfile>), ApiError> {
    let user = User::from(new_user)
        .insert(&pool).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub(super) async fn read(
    Extension(pool): Extension<PgPool>,
    Query(AnyId{id}): Query<AnyId>
) -> Result<(StatusCode, Json<UserProfile>), ApiError> {
    let user = UserProfile::select(id, &pool).await?;

    Ok((StatusCode::OK, Json(user)))
}

pub(super) async fn update(
    Extension(pool): Extension<PgPool>,
    Json(user): Json<User>
) -> Result<StatusCode, ApiError> {
    user.update(&pool).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub(super) async fn delete(
    Extension(pool): Extension<PgPool>,
    Query(AnyId{id}): Query<AnyId>
) -> Result<StatusCode, ApiError> {
    User::delete(id, &pool).await?;

    Ok(StatusCode::NO_CONTENT)
}


impl From<NewUser> for User {
    fn from(value: NewUser) -> Self {
        User {
            id: Uuid::new_v4(),
            email: value.email,
            username: value.username,
            password: value.password,
        }
    }
}
