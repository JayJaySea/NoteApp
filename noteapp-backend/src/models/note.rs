use serde::{Serialize, Deserialize};
use sqlx::{self, PgPool, FromRow};
use uuid::Uuid;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: Uuid,
    pub label: Option<String>,
    pub contents: String,
    pub user_id: Uuid
}

impl Note {
    pub async fn insert(self, pool: &PgPool) 
    -> Result<Self, sqlx::Error> {
        let note = sqlx::query_as(
            "INSERT INTO notes VALUES ($1, $2, $3, $4) 
             RETURNING id, label, contents, user_id"
            )
            .bind(self.id)
            .bind(self.label)
            .bind(self.contents)
            .bind(self.user_id)
            .fetch_one(pool)
            .await?;

        Ok(note)
    }

    pub async fn select_user_notes(user_id: Uuid, pool: &PgPool) 
    -> Result<Vec<Self>, sqlx::Error> {
        let notes = sqlx::query_as(
            "SELECT * FROM notes where user_id = $1"
            )
            .bind(user_id)
            .fetch_all(pool)
            .await?;

        Ok(notes)
    }

    pub async fn update(self, pool: &PgPool) 
    -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE notes SET label = $1, contents = $2 
             WHERE id = $3 AND user_id = $4"
            )
            .bind(self.label)
            .bind(self.contents)
            .bind(self.id)
            .bind(self.user_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn delete(id: Uuid, user_id: Uuid, pool: &PgPool) 
    -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM notes WHERE id = $1 AND user_id = $2"
            )
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
