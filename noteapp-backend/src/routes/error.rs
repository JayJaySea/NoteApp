use thiserror::Error;
use serde_json::json;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json
};

#[allow(unused_imports)]
use tracing::{trace, debug, info, warn, error};


#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Invalid token")]
    InvalidToken(jsonwebtoken::errors::Error),

    #[error("No token on protected resource")]
    NoToken,

    #[error("Invalid scope for protected resource")]
    InvalidScope,

    #[error("Specified resource was not found")]
    NotFound,

    #[error("Server refuses to brew coffee because it is, permanently, a teapot")]
    ImATeapot,

    #[error("User with specified username or email exists")]
    UniqueUserViolation(String),

    #[error("User tried to modify resource they dont own")]
    NotOwner,

    #[error("Owner of resource tried to delete their membership of that resource")]
    IsOwner,

    #[error(transparent)]
    Generic(#[from] anyhow::Error),

    #[error("Unknown error")]
    Unknown(String)
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        warn!("Error: {self:?}");

        let tmp;
                

        let (status, error_message) = match self {
            Self::UniqueUserViolation(column) => {
                tmp = format!("User with this `{column}` already exists");
                (StatusCode::CONFLICT, tmp.as_str())
            }
            Self::InvalidCredentials => 
                (StatusCode::UNAUTHORIZED, "Invalid Credentials"),
            Self::InvalidToken(_) => 
                (StatusCode::BAD_REQUEST, "Invalid token"),
            Self::InvalidScope => 
                (StatusCode::FORBIDDEN, "Invalid user scope when accessing protected resource"),
            Self::Generic(_) => 
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
            Self::Unknown(_) => 
                (StatusCode::INTERNAL_SERVER_ERROR, "Unknown Error"),
            Self::ImATeapot => 
                (StatusCode::IM_A_TEAPOT, "Server refuses to brew coffee because it is, permanently, a teapot"),
            Self::NotFound => 
                (StatusCode::NOT_FOUND, "Specified resource was not found in the database"),
            Self::NotOwner => 
                (StatusCode::FORBIDDEN, "Tried to modify resource that is not owned by token subject"),
            Self::IsOwner => 
                (StatusCode::FORBIDDEN, "Tried to delete membership of a resource without passing it's ownership"),
            Self::NoToken => 
                (StatusCode::UNAUTHORIZED, "Token not provided when accessing protected resource"),
        };

        let body = Json(json!({ "error": error_message }));

        (status, body).into_response()
    }
}
