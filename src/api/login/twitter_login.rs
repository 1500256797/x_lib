use std::fs::File;
use std::io::Write;
use crate::traits::IntoRequestBuilder;
use reqwest::{Client, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct TwitterLoginRequest {
    pub user_name: String,
    pub password: String,
    pub cookie_file_path: Option<String>,
}

pub struct GetFlowTokenRequest {
    pub guest_token: String,
    pub bearer_token: String,
    pub body: serde_json::Value,
}
#[derive(Deserialize)]
pub struct ApiError {
    pub code: i64,
    pub message: String,
}
#[derive(Deserialize)]
pub struct User {
    pub id: i64,
    pub id_str: String,
    pub name: String,
    pub screen_name: String,
}
#[derive(Deserialize)]
pub struct OpenAccount {
    pub user: Option<User>,
    pub next_link: Option<Link>,
    pub attribution_event: Option<String>,
}

#[derive(Deserialize)]
pub struct Subtask {
    pub subtask_id: String,
    pub open_account: Option<OpenAccount>,
}

#[derive(Deserialize)]
pub struct Link {
    pub link_type: String,
    pub link_id: String,
}

#[derive(Deserialize)]
pub struct GuestToken {
    pub guest_token: String,
}

#[derive(Deserialize)]
pub struct VerifyCredentials {
    pub errors: Option<Vec<ApiError>>,
}

#[derive(Deserialize)]
pub struct Insrumentation {
    pub url: String,
    pub timeout_ms: i64,
    pub next_link: Link,
}
#[derive(Deserialize)]
pub struct Flow {
    pub errors: Option<Vec<ApiError>>,
    pub flow_token: String,
    pub status: String,
    pub subtasks: Vec<Subtask>,
    pub js_instrumentation: Option<Insrumentation>,
}


impl IntoRequestBuilder for GetFlowTokenRequest {
    fn into_request(self, client: reqwest::Client) -> reqwest::RequestBuilder {
        client
            .post("https://api.twitter.com/1.1/onboarding/task.json")
            .header("Authorization", format!("Bearer {}", self.bearer_token))
            .header("Content-Type", "application/json")
            .header("User-Agent", "TwitterAndroid/99")
            .header("X-Guest-Token", self.guest_token.replace("\"", ""))
            .header("X-Twitter-Auth-Type", "OAuth2Client")
            .header("X-Twitter-Active-User", "yes")
            .header("X-Twitter-Client-Language", "en")
            .json(&self.body)
    }
}

pub struct GetGuestTokenRequest {
    pub bearer_token: String,
}

impl IntoRequestBuilder for GetGuestTokenRequest {
    fn into_request(self, client: reqwest::Client) -> reqwest::RequestBuilder {
        client
            .post("https://api.twitter.com/1.1/guest/activate.json")
            .header("Authorization", format!("Bearer {}", self.bearer_token))
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CookieData {
    pub name: String,
    pub value: String,
}

pub fn save_cookies_to_file(
    response: &Response,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get the cookies from the response
    let cookies = response.cookies();
    // Serialize the cookies into JSON format
    // Convert cookies into a serializable data structure
    let cookie_data: Vec<CookieData> = cookies
        .map(|cookie| CookieData {
            name: cookie.name().to_string(),
            value: cookie.value().to_string(),
            // Add any other fields you need from the cookie
        })
        .collect();

    // Serialize the cookie data into JSON format
    let serialized_cookies = serde_json::to_string(&cookie_data)?;
    // Open the file for writing
    let mut file = File::create(file_path)?;
    // Write the serialized cookies to the file
    file.write_all(serialized_cookies.as_bytes())?;

    Ok(())
}