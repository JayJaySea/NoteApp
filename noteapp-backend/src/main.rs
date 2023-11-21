use std::error::Error;

use noteapp_backend::setup_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let app = setup_app().await;

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
