use serde::Serialize;
use sqlx::{self, PgPool, FromRow};
use uuid::Uuid;

#[derive(FromRow, Serialize)]
pub struct Note {
    pub id: Uuid,
    pub label: Option<String>,
    pub contents: String,
}

impl Note {
    pub async fn insert(self, pool: &PgPool) -> Result<Self, sqlx::Error> {
        let note = sqlx::query_as(
            "INSERT INTO notes VALUES ($1, $2, $3) 
             RETURNING id, label, contents"
            )
            .bind(self.id)
            .bind(self.label)
            .bind(self.contents)
            .fetch_one(pool)
            .await?;

        println!("lo");
        Ok(note)

    }
}

