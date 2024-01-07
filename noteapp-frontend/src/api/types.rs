use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
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
pub struct UpdatePassword {
    pub password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
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
    pub id: Uuid,
    pub label: String,
    pub contents: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Token {
    pub status: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Error {
    pub error: String,
}


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AnyPassword {
    pub password: String,
}
