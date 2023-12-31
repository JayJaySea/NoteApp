use argon2::Argon2;
use password_hash::{SaltString, PasswordHash, PasswordVerifier};
use serde::{Serialize, Deserialize};
use sqlx::{self, PgPool, FromRow};
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub email: String,
    pub username: String
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub id: Uuid,
    pub email: String,
    pub password: String,
}

impl UserProfile {
    pub async fn select(id: Uuid, pool: &PgPool) 
    -> Result<Self, sqlx::Error> {
        let user = sqlx::query_as(
            "SELECT id, email, username FROM users WHERE id = $1"
            )
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }
}

#[derive(FromRow, Debug, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
}

impl User {
    pub async fn insert(self, pool: &PgPool) 
    -> Result<UserProfile, sqlx::Error> {
        let user = sqlx::query_as(
            "INSERT INTO users VALUES ($1, $2, $3, $4) 
             RETURNING id, email, username"
            )
            .bind(self.id)
            .bind(self.email)
            .bind(self.username)
            .bind(self.password)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn update(self, pool: &PgPool) 
    -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET email = $1, username = $2
             WHERE id = $3"
            )
            .bind(self.email)
            .bind(self.username)
            .bind(self.id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn update_password(id: Uuid, new_pass: String, pool: &PgPool)
    -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET password = $1
             WHERE id = $2"
            )
            .bind(new_pass)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn delete(id: Uuid, pool: &PgPool) 
    -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM users WHERE id = $1"
            )
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn select_credentials(email: &str, pool: &PgPool) 
    -> Result<Credentials, sqlx::Error> {
        let credentials: Credentials = sqlx::query_as(
            "SELECT id, email, password FROM users WHERE email = $1"
            )
            .bind(email)
            .fetch_one(pool)
            .await?;

        Ok(credentials)
    }

    pub async fn select_credentials_by_id(id: Uuid, pool: &PgPool) 
    -> Result<Credentials, sqlx::Error> {
        let credentials: Credentials = sqlx::query_as(
            "SELECT id, email, password FROM users WHERE id = $1"
            )
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(credentials)
    }
}
