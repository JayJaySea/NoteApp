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
    pub async fn insert(self, pool: &PgPool) 
    -> Result<Self, sqlx::Error> {
        let note = sqlx::query_as(
            "INSERT INTO notes VALUES ($1, $2, $3) 
             RETURNING id, label, contents"
            )
            .bind(self.id)
            .bind(self.label)
            .bind(self.contents)
            .fetch_one(pool)
            .await?;

        Ok(note)
    }

    pub async fn select_all(pool: &PgPool) 
    -> Result<Vec<Self>, sqlx::Error> {
        let notes = sqlx::query_as(
            "SELECT * FROM notes"
            )
            .fetch_all(pool)
            .await?;

        Ok(notes)
    }

    pub async fn select(id: Uuid, pool: &PgPool) 
    -> Result<Self, sqlx::Error> {
        let note = sqlx::query_as(
            "SELECT * FROM notes WHERE id = $1"
            )
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(note)
    }

    pub async fn delete(id: Uuid, pool: &PgPool) 
    -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM notes WHERE id = $1"
            )
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
