use argon2::Argon2;
use axum::{Json, Extension, extract::Query};
use http::StatusCode;
use password_hash::{SaltString, PasswordHasher};
use rand_core::OsRng;
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
    let user = User::try_from(new_user)?
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
    Json(mut user): Json<User>
) -> Result<StatusCode, ApiError> {
    user.password = hash_password(&user.password)?;
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


impl TryFrom<NewUser> for User {
    type Error = ApiError;
    fn try_from(value: NewUser) -> Result<Self, Self::Error> {
        let user = User {
            id: Uuid::new_v4(),
            email: value.email,
            username: value.username,
            password: hash_password(&value.password)?
        };

        Ok(user)
    }
}

fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| ApiError::Unknown(e.to_string()))?
        .to_string();

    Ok(hash)
}
