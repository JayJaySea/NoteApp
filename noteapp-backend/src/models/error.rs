use sqlx;

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("This record was not found in the database")]
    NotFound,

    #[error("Unique constraint failed {0}")]
    Collision(String),

    #[error("Operation on reference failed {0}")]
    InvalidReferenceOperation(String),

    #[error(transparent)]
    Sqlx(sqlx::Error),

    #[error(transparent)]
    Generic(#[from] anyhow::Error)
}
