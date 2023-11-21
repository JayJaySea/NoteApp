use axum::{Router, Extension};
use init_db::setup_database;
use tower::ServiceBuilder;

pub mod routes;
pub mod models;
pub mod init_db;

fn setup_endpoints() -> Router {
    Router::new()
        .merge(routes::user_routes())
        .merge(routes::note_routes())
}

pub async fn setup_app() -> Router {
    let pool = setup_database().await;

    setup_endpoints()
        .layer(ServiceBuilder::new()
               .layer(Extension(pool)))
}
