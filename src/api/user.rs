use gloo_net::http::Request;
use gloo_net::Error;

use serde::Deserialize;
use serde_json::json;

use super::APP_HOST;

#[derive(Deserialize)]
pub struct UserLoginResponse {
  pub token: String
}

pub async fn login (username: String, password: String) -> Result<UserLoginResponse, Error> {
  let response = Request::post(&format!("{}/login", APP_HOST))
      .json(&json!({
        "username": username,
        "password": password
      }))?
      .send()
      .await?;
  
  response.json::<UserLoginResponse>().await
}

#[derive(Deserialize)]
pub struct UserInfoResponse {
  pub id: i32,
  pub username: String,
  pub created_at: String
}


pub async fn info (token: &String) -> Result<UserInfoResponse, Error> {
  let response = Request::get(&format!("{}/user/info", APP_HOST))
      .header("Authroization", &format!("Bearer {}", token))
      .send()
      .await?;
  
  response.json::<UserInfoResponse>().await
}

