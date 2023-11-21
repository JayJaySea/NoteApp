use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Note {
    id: Uuid,
    label: Option<String>,
    contents: String,
}


