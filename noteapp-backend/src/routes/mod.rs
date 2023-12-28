use axum::{
    routing::{
        post,
        get,
        put,
        delete
    },
    body::boxed,
    Router, response::Response
};
use http::StatusCode;
use hyper::Body;
use tower::ServiceExt;
use tower_http::services::ServeDir;
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
        .route("/api/note",  post(note::create))
        .route("/api/notes", get(note::read_all))
        .route("/api/note",  put(note::update))
        .route("/api/note",  delete(note::delete))
}

pub fn user_routes() -> Router {
    Router::new()
        .route("/api/register",  post(user::create))
        .route("/api/user",      get(user::read))
        .route("/api/user",      put(user::update))
        .route("/api/user",      delete(user::delete))
        .route("/api/login",     post(user::login))
}

pub fn static_routes() -> Router {
    Router::new()
        .fallback_service(get(|req| async move {
            match ServeDir::new("/dist").oneshot(req).await {
                Ok(res) => res.map(boxed),
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
            }
        }))
}
