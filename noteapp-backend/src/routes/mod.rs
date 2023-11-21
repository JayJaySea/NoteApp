use axum::{
    routing::post,
    Router
};

pub mod note;
pub mod user;
pub mod error;


pub fn note_routes() -> Router {
    Router::new()
        .route("/note", post(note::create_note))
}

pub fn user_routes() -> Router {
    Router::new()
}
