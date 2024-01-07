use axum::{Json, Extension, extract::Query, response::{Response, IntoResponse}};
use axum_extra::extract::CookieJar;
use http::{StatusCode, header, Request, HeaderMap};
use hyper::Body;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;
use crate::{models::user::{User, UserProfile, Credentials}, auth::{hash_password, authenticate, generate_token, generate_auth_cookie, extract_token, decode_token, generate_bad_cookie}};

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

#[derive(Deserialize, Debug)]
pub(super) struct UpdatePasswordRequest {
    pub password: String,
    pub new_password: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct AnyPassword {
    pub password: String,
}


pub(super) async fn create(
    Extension(pool): Extension<PgPool>,
    Json(new_user): Json<UserRequest>
) -> Result<(StatusCode, Json<UserProfile>), ApiError> {
    let mut user = User::from(new_user);
    user.password = hash_password(&user.password)?;
    let user = user.insert(&pool).await?;

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
    let mut user = User::from(user);
    user.id = claims.sub;

    let credentials = User::select_credentials_by_id(user.id, &pool).await?;
    
    if authenticate(&user.password, &credentials.password).await? {
        user.update(&pool).await?;
        return Ok(StatusCode::NO_CONTENT);
    };

    Err(ApiError::InvalidCredentials)
}

pub(super) async fn update_password(
    cookie_jar: CookieJar,
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    Json(passwords): Json<UpdatePasswordRequest>
) -> Result<StatusCode, ApiError> {
    let token = extract_token(cookie_jar, &headers)?;
    let claims = decode_token(&token)?;

    let credentials = User::select_credentials_by_id(claims.sub, &pool).await?;
    
    if authenticate(&passwords.password, &credentials.password).await? {
        User::update_password(claims.sub, hash_password(&passwords.new_password)?, &pool).await?;
        return Ok(StatusCode::NO_CONTENT);
    };

    Err(ApiError::InvalidCredentials)
}

pub(super) async fn delete(
    cookie_jar: CookieJar,
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    Json(AnyPassword{password}): Json<AnyPassword>
) -> Result<StatusCode, ApiError> {
    let token = extract_token(cookie_jar, &headers)?;
    let claims = decode_token(&token)?;

    let credentials = User::select_credentials_by_id(claims.sub, &pool).await?;
    if authenticate(&password, &credentials.password).await? {
        User::delete(claims.sub, &pool).await?;
        return Ok(StatusCode::NO_CONTENT);
    };

    Err(ApiError::InvalidCredentials)
}

pub (super) async fn login(
    Extension(pool): Extension<PgPool>,
    Json(LoginRequest{email, password}): Json<LoginRequest>
) -> Result<impl IntoResponse, ApiError> {
    let credentials = User::select_credentials(&email, &pool).await
        .map_err(|_| ApiError::InvalidCredentials)?;
    
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

pub(super) async fn logout(
    cookie_jar: CookieJar,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let token = extract_token(cookie_jar, &headers)?;
    let _ = decode_token(&token)?;

    let cookie = generate_bad_cookie("");

    let mut response = Response::new("".to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    return Ok(response);
}

impl From<UserRequest> for User {
    fn from(value: UserRequest) -> Self {
        User {
            id: Uuid::new_v4(),
            email: value.email,
            username: value.username,
            password: value.password
        }
    }
}
