use super::types::{Error, User, LoginUser, UpdateUser, CreateUser};
use reqwasm::http;

pub async fn register_user(user_data: CreateUser) -> Result<User, String> {
    let user_data = serde_json::to_string(&user_data)
        .map_err(|_| "Failed to parse request")?;

    let response = match http::Request::post("http://localhost:8080/api/register")
        .header("Content-Type", "application/json")
        .body(user_data)
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

    let res_json = response.json::<User>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}
