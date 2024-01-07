use argon2::Argon2;
use hyper::Body;
use time;
use jsonwebtoken as jwt;
use axum_extra::extract::{cookie::{Cookie, SameSite}, CookieJar};
use axum::{async_trait, extract::FromRequestParts, TypedHeader, headers::{Authorization, authorization::Bearer, }, response::IntoResponseParts, RequestPartsExt};
use http::{request::Parts, Request, header, HeaderMap, HeaderValue};
use jsonwebtoken::{EncodingKey, DecodingKey, Validation};
use jwt::{Header, decode};
use once_cell::sync::Lazy;
use password_hash::{SaltString, PasswordHasher, PasswordHash, PasswordVerifier};
use rand_core::OsRng;
use tracing::info;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::routes::error::ApiError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub async fn authenticate(password: &str, hash: &str) 
-> Result<bool, ApiError> {
    let hash = PasswordHash::new(hash).unwrap();

    let password_ok = Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .map(|_| true)
        .or_else(|e| match e {
            password_hash::Error::Password => Ok(false),
            e => Err(ApiError::Unknown(e.to_string()))
        })?;

    Ok(password_ok)
}

pub fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| ApiError::Unknown(e.to_string()))?
        .to_string();

    Ok(hash)
}

pub fn generate_token(id: Uuid) -> String {
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims = Claims {
        sub: id,
        exp,
        iat,
    };

    let token = jwt::encode(
        &Header::default(),
        &claims,
        &KEYS.encoding,
    )
    .unwrap();

    token
}

pub fn generate_auth_cookie(token: &str) -> Cookie {
    Cookie::build("token", token)
        .path("/")
        .max_age(time::Duration::minutes(60))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish()
}

pub fn generate_bad_cookie(bad_token: &str) -> Cookie {
    Cookie::build("token", bad_token)
        .path("/")
        .max_age(time::Duration::minutes(-60))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish()
}

pub fn extract_token(cookie_jar: CookieJar, headers: &HeaderMap<HeaderValue>) -> Result<String, ApiError> {
    cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            headers
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        })
        .ok_or(ApiError::NoToken)
}

pub fn decode_token(token: &str) -> Result<Claims, ApiError>{
    let claims = decode::<Claims>(
        &token,
        &KEYS.decoding,
        &Validation::default(),
    )
    .map_err(|e| ApiError::InvalidToken(e))?
    .claims;

    Ok(claims)
}
