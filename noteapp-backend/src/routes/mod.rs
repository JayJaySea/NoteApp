use axum::{
    routing::{
        post,
        get,
        delete
    },
    Router
};
use uuid::Uuid;
use serde::Deserialize;

pub mod note;
pub mod user;
pub mod error;

#[derive(Deserialize, Debug)]
pub struct AnyId {
    pub id: Uuid
}

pub fn note_routes() -> Router {
    Router::new()
        .route("/note", post(note::create))
        .route("/note", get(note::read))
        .route("/note", delete(note::delete))
        .route("/notes", get(note::read_all))
}

pub fn user_routes() -> Router {
    Router::new()
}
