use super::types::{Note, Error, CreateNote, UpdateNote};
use reqwasm::http;
use uuid::Uuid;

pub async fn get_notes() -> Result<Vec<Note>, String> {
    let response = match http::Request::get("/api/notes")
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<Error>().await;
        match error_response {
            Ok(error) => return Err(error.error),
            _ => return Err(format!("API error: {}", response.status())),
        }
    }

    let res_json = response.json::<Vec<Note>>().await;
    match res_json {
        Ok(notes) => Ok(notes),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn add_note(note_data: CreateNote) -> Result<Note, String> {
    let note_data = serde_json::to_string(&note_data)
        .map_err(|_| "Failed to parse request")?;

    let response = match http::Request::post("/api/note")
        .header("Content-Type", "application/json")
        .body(note_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 201 {
        let error_response = response.json::<Error>().await;
        match error_response {
            Ok(error) => return Err(error.error),
            _ => return Err(format!("API error: {}", response.status())),
        }
    }

    let res_json = response.json::<Note>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn delete_note(id: Uuid) -> Result<(), String> {
    let response = match http::Request::delete(&format!("/api/note?id={}", id))
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 204 {
        let error_response = response.json::<Error>().await;
        match error_response {
            Ok(error) => return Err(error.error),
            _ => return Err(format!("API error: {}", response.status())),
        }
    }

    Ok(())
}

pub async fn update_note(note_data: UpdateNote) -> Result<(), String> {
    let note_data = serde_json::to_string(&note_data)
        .map_err(|_| "Failed to parse request")?;

    let response = match http::Request::put("/api/note")
        .header("Content-Type", "application/json")
        .body(note_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 204 {
        let error_response = response.json::<Error>().await;
        match error_response {
            Ok(error) => return Err(error.error),
            _ => return Err(format!("API error: {}", response.status())),
        }
    }

    Ok(())
}
