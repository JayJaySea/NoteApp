use std::{path::PathBuf, fs};

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
        .route("/api/user/register",    post(user::create))
        .route("/api/user/login",       post(user::login))
        .route("/api/user/logout",      post(user::logout))
        .route("/api/user/password",    put(user::update_password))
        .route("/api/user",             get(user::read))
        .route("/api/user",             put(user::update))
        .route("/api/user",             delete(user::delete))
}

pub fn static_routes() -> Router {
    Router::new()
        .fallback_service(get(|req| async move {
            match ServeDir::new("/dist").oneshot(req).await {
                Ok(res) => {
                    let status = res.status();
                    match status {
                        StatusCode::NOT_FOUND => {
                            let index_path = PathBuf::from("/dist").join("index.html");
                            let index_content = match fs::read_to_string(index_path) {
                                Err(_) => {
                                    return Response::builder()
                                        .status(StatusCode::NOT_FOUND)
                                        .body(boxed(Body::from("index file not found")))
                                        .unwrap()
                                }
                                Ok(index_content) => index_content,
                            };
    
                            Response::builder()
                                .status(StatusCode::OK)
                                .body(boxed(Body::from(index_content)))
                                .unwrap()
                        }
                        _ => res.map(boxed),
                    }
                }
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
        }}))
}
