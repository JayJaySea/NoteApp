use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UpdateUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Note {
    pub id: Uuid,
    pub label: String,
    pub contents: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CreateNote {
    pub label: String,
    pub contents: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UpdateNote {
    pub label: String,
    pub contents: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Error {
    pub error: String,
}
