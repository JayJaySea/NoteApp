use axum::{Json, Extension, extract::Query, response::{Response, IntoResponse}};
use axum_extra::extract::CookieJar;
use http::{StatusCode, header, Request, HeaderMap};
use hyper::Body;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;
use crate::{models::user::{User, UserProfile, Credentials}, auth::{hash_password, authenticate, generate_token, generate_auth_cookie, extract_token, decode_token}};

use super::{error::ApiError, AnyId};

#[derive(Deserialize, Debug)]
pub(super) struct UserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub(super) async fn create(
    Extension(pool): Extension<PgPool>,
    Json(new_user): Json<UserRequest>
) -> Result<(StatusCode, Json<UserProfile>), ApiError> {
    let user = User::try_from(new_user)?
        .insert(&pool).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub(super) async fn read(
    cookie_jar: CookieJar,
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>
) -> Result<(StatusCode, Json<UserProfile>), ApiError> {
    let token = extract_token(cookie_jar, &headers)?;
    let claims = decode_token(&token)?;

    let user = UserProfile::select(claims.sub, &pool).await?;

    Ok((StatusCode::OK, Json(user)))
}

pub(super) async fn update(
    cookie_jar: CookieJar,
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    Json(user): Json<UserRequest>
) -> Result<StatusCode, ApiError> {
    let token = extract_token(cookie_jar, &headers)?;
    let claims = decode_token(&token)?;
    let mut user = User::try_from(user)?;

    user.id = claims.sub;
    user.password = hash_password(&user.password)?;
    user.update(&pool).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub(super) async fn delete(
    cookie_jar: CookieJar,
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>
) -> Result<StatusCode, ApiError> {
    let token = extract_token(cookie_jar, &headers)?;
    let claims = decode_token(&token)?;

    User::delete(claims.sub, &pool).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub (super) async fn login(
    Extension(pool): Extension<PgPool>,
    Json(LoginRequest{email, password}): Json<LoginRequest>
) -> Result<impl IntoResponse, ApiError> {
    let credentials = User::select_credentials(&email, &pool).await?;
    
    if authenticate(&password, &credentials.password).await? {
        let token = generate_token(credentials.id);
        let cookie = generate_auth_cookie(&token);

        let mut response = Response::new(json!({"status": "success", "token": token}).to_string());

        response
            .headers_mut()
            .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

        return Ok(response);
    }

    return Err(ApiError::InvalidCredentials)
}


impl TryFrom<UserRequest> for User {
    type Error = ApiError;
    fn try_from(value: UserRequest) -> Result<Self, Self::Error> {
        let user = User {
            id: Uuid::new_v4(),
            email: value.email,
            username: value.username,
            password: hash_password(&value.password)?
        };

        Ok(user)
    }
}
