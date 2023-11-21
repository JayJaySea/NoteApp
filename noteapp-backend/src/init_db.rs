use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn setup_database() -> PgPool {
    let conn = setup_conn().await;

    conn
}

async fn setup_conn() -> PgPool {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run pending migrations!");

    pool
}
