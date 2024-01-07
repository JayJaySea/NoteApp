use super::types::{Error, User, LoginUser, UpdateUser, CreateUser, Token, UpdatePassword, AnyPassword};
use reqwasm::http;

pub async fn register_user(user_data: CreateUser) -> Result<User, String> {
    let user_data = serde_json::to_string(&user_data)
        .map_err(|_| "Failed to parse request")?;

    let response = match http::Request::post("/api/user/register")
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

pub async fn login_user(user_data: LoginUser) -> Result<Token, String> {
    let user_data = serde_json::to_string(&user_data)
        .map_err(|_| "Failed to parse request")?;

    let response = match http::Request::post("/api/user/login")
        .header("Content-Type", "application/json")
        .body(user_data)
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

    let res_json = response.json::<Token>().await;
    match res_json {
        Ok(token) => Ok(token),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn get_user() -> Result<User, String> {
    let response = match http::Request::get("/api/user")
        .header("Content-Type", "application/json")
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

    let res_json = response.json::<User>().await;
    match res_json {
        Ok(user) => Ok(user),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn update_user(user_data: UpdateUser) -> Result<(), String> {
    let user_data = serde_json::to_string(&user_data)
        .map_err(|_| "Failed to parse request")?;

    let response = match http::Request::put("/api/user")
        .header("Content-Type", "application/json")
        .body(user_data)
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

pub async fn update_password(pass_data: UpdatePassword) -> Result<(), String> {
    let pass_data = serde_json::to_string(&pass_data)
        .map_err(|_| "Failed to parse request")?;

    let response = match http::Request::put("/api/user/password")
        .header("Content-Type", "application/json")
        .body(pass_data)
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

pub async fn delete_user(pass: AnyPassword) -> Result<(), String> {
    let pass = serde_json::to_string(&pass)
        .map_err(|_| "Failed to parse request")?;

    let response = match http::Request::delete("/api/user")
        .header("Content-Type", "application/json")
        .body(pass)
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

pub async fn logout_user() -> Result<(), String> {
    let response = match http::Request::post("/api/user/logout")
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

    Ok(())
}
