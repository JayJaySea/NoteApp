use axum::{
    routing::{
        post,
        get,
        put,
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
        .route("/note",  post(note::create))
        .route("/note",  get(note::read))
        .route("/note",  put(note::update))
        .route("/note",  delete(note::delete))
        .route("/notes", get(note::read_all))
}

pub fn user_routes() -> Router {
    Router::new()
        .route("/register",  post(user::create))
        .route("/user",      get(user::read))
        .route("/user",      put(user::update))
        .route("/user",      delete(user::delete))
}
