pub mod traits;
pub mod api;




use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use crate::api::homepage::user_home_page_content_request::GetUserHomePageContentRequest;
use crate::api::homepage::user_home_page_content_response::UserHomePageContentResponse;
use crate::api::login::twitter_login::{CookieData, Flow, GetFlowTokenRequest, GetGuestTokenRequest, save_cookies_to_file, TwitterLoginRequest, VerifyCredentials};
use crate::traits::{IntoRequestBuilder, SendRequestAndLog};
use anyhow::{anyhow, Result};
use api::*;
use reqwest::{Client, ClientBuilder, RequestBuilder, Response, Url};
use serde_json::json;
use crate::api::follow::twitter_follow_relation_request::GetUserFollowersListRequest;
use crate::api::follow::twitter_followers_list_response::FollowersListResp;
use crate::api::homepage::user_by_screen_name_request::UserByScreenNameRequest;
use crate::api::homepage::user_by_screen_name_response::UserByScreenNameResponse;

pub const LOGIN_URL: &str = "https://api.twitter.com/1.1/onboarding/task.json";
pub const LOGOUR_URL: &str = "https://api.twitter.com/1.1/account/logout.json";
pub const GUEST_ACTIVE_URL: &str = "https://api.twitter.com/1.1/guest/activate.json";
pub const VERIFY_CREDENTIALS_URL: &str =
    "https://api.twitter.com/1.1/account/verify_credentials.json";
pub const OAUTH_URL: &str = "https://api.twitter.com/oauth2/token";
pub const BEARER_TOKEN: &str = "AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA";
pub const APP_CONSUMER_KEY: &str = "3nVuSoBZnx6U4vzUxf5w";
pub const APP_CONSUMER_SECRET: &str = "Bcs59EFbbsdF6Sl9Ng71smgStWEGwXXKSjYvPVt7qys";
#[derive(Debug, Clone)]
pub struct ReAPI {
    pub client: Client,
    pub guest_token: String,
    pub csrf_token: String,
}

fn parse_flow(flow: Flow) -> Result<String, anyhow::Error> {
    if flow.subtasks.len() > 0 {
        let subtask_id = flow.subtasks[0].subtask_id.as_str();
        match subtask_id {
            "LoginEnterAlternateIdentifierSubtask"
            | "LoginAcid"
            | "LoginTwoFactorAuthChallenge"
            | "DenyLoginSubtask" => {
                return Err(anyhow!("Login failed"));
            }
            _ => {
                return Ok(flow.flow_token);
            }
        }
    }
    Ok(flow.flow_token)
}
impl SendRequestAndLog for RequestBuilder {
    async fn send_request_and_log(self) -> Result<Response> {
        let res = self.send().await?;
        let status = res.status();
        // match status code
        match status.as_u16() {
            200 => {
                println!("200 OK");
            }
            400 => {
                println!("400 Bad Request");
            }
            401 => {
                println!("401 Unauthorized");
            }
            403 => {
                println!("403 Forbidden");
            }
            404 => {
                println!("404 Not Found");
            }
            500 => {
                println!("500 Internal Server Error");
            }
            _ => {
                println!("Other");
            }
        }

        if status.is_client_error() || status.is_server_error() {
            let text = res.text().await?;
            return Err(anyhow!("API failed: {}", text));
        }
        Ok(res)
    }
}

impl ReAPI {
    pub fn new() -> ReAPI {
        let client = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .build()
            .unwrap();
        return ReAPI {
            client,
            csrf_token: String::from(""),
            guest_token: String::from(""),
        };
    }
    // get user id by screen name without login
    pub async fn get_user_id_by_screen_name(
        &self,
        req: UserByScreenNameRequest,
    ) -> Result<UserByScreenNameResponse> {
        let req = req.into_request(self.client.clone());
        let res = req.send_request_and_log().await?;
        let text = res.text().await?;
        let res: UserByScreenNameResponse = serde_json::from_str(&text)?;
        Ok(res)
    }


    // user login
    pub async fn login_in(&mut self, login_req: TwitterLoginRequest) -> Result<(), anyhow::Error> {
        let cookie_file_path = login_req.cookie_file_path.unwrap_or("cookies.json".to_string());
        // get guest token
        let get_guest_token_req = GetGuestTokenRequest {
            bearer_token: BEARER_TOKEN.to_string(),
        };

        let req = get_guest_token_req.into_request(self.client.clone());
        let res = req.send_request_and_log().await?;
        save_cookies_to_file(&res,cookie_file_path.as_str()).map_err(|e| anyhow!("save cookies failed: {}", e))?;
        let op = res.json::<serde_json::Value>().await?;
        let guest_token = op.get("guest_token").unwrap().to_string();


        // flow start
        let data = json!(
            {
                "flow_name": "login",
                "input_flow_data": {
                    "flow_context" : {
                        "debug_overrides": {},
                        "start_location": {
                            "location": "splash_screen"
                        }
                    }
                }
            }
        );
        let get_flow_token_req = GetFlowTokenRequest {
            guest_token: guest_token.clone(),
            bearer_token: BEARER_TOKEN.to_string(),
            body: data,
        };
        let req = get_flow_token_req.into_request(self.client.clone());
        let res = req.send_request_and_log().await?;
        save_cookies_to_file(&res,cookie_file_path.as_str()).map_err(|e| anyhow!("save cookies failed: {}", e))?;
        let cookies = res.cookies();
        let mut csrf_token = "".to_string();
        for cookie in cookies {
            if cookie.name().eq("ct0") {
                csrf_token = cookie.value().to_string()
            }
        }
        let flow: Flow = res.json().await?;


        let flow_token = parse_flow(flow)?;
        // flow instrumentation step
        let data = json!(
            {
                "flow_token": flow_token,
                "subtask_inputs" : [{
                    "subtask_id": "LoginJsInstrumentationSubtask",
                    "js_instrumentation":{
                        "response": "{}",
                        "link": "next_link"
                    }
                }],
            }
        );
        let get_flow_token_req = GetFlowTokenRequest {
            guest_token: guest_token.clone(),
            bearer_token: BEARER_TOKEN.to_string(),
            body: data,
        };
        let req = get_flow_token_req.into_request(self.client.clone());
        let res = req.send_request_and_log().await?;
        save_cookies_to_file(&res,cookie_file_path.as_str()).map_err(|e| anyhow!("save cookies failed: {}", e))?;
        let flow: Flow = res.json().await?;
        let flow_token = parse_flow(flow)?;

        // flow username step
        let data = json!(
            {
                "flow_token": flow_token,
                "subtask_inputs" : [{
                    "subtask_id": "LoginEnterUserIdentifierSSO",
                    "settings_list": {
                        "setting_responses" : [{
                            "key":           "user_identifier",
                            "response_data": {
                                "text_data" :{
                                    "result": login_req.user_name.clone()
                                }
                            }
                        }],
                        "link": "next_link"
                    }
                }]
            }
        );
        let get_flow_token_req = GetFlowTokenRequest {
            guest_token: guest_token.clone(),
            bearer_token: BEARER_TOKEN.to_string(),
            body: data,
        };
        let req = get_flow_token_req.into_request(self.client.clone());
        let res = req.send_request_and_log().await?;
        save_cookies_to_file(&res,cookie_file_path.as_str()).map_err(|e| anyhow!("save cookies failed: {}", e))?;
        let flow: Flow = res.json().await?;
        let flow_token = parse_flow(flow)?;

        // flow password step
        // flow password step
        let data = json!(
            {
                "flow_token": flow_token,
                "subtask_inputs": [{
                    "subtask_id":     "LoginEnterPassword",
                    "enter_password": {
                        "password": login_req.password.clone(),
                        "link": "next_link"
                    },
                }]
            }
        );
        let get_flow_token_req = GetFlowTokenRequest {
            guest_token: guest_token.clone(),
            bearer_token: BEARER_TOKEN.to_string(),
            body: data,
        };
        let req = get_flow_token_req.into_request(self.client.clone());
        let res = req.send_request_and_log().await?;
        save_cookies_to_file(&res,cookie_file_path.as_str()).map_err(|e| anyhow!("save cookies failed: {}", e))?;
        let flow: Flow = res.json().await?;
        let flow_token = parse_flow(flow)?;

        // flow duplication check
        let data = json!(
            {
                "flow_token": flow_token,
                "subtask_inputs": [{
                    "subtask_id":              "AccountDuplicationCheck",
                    "check_logged_in_account": {
                        "link": "AccountDuplicationCheck_false"
                    },
                }]
            }
        );
        let get_flow_token_req = GetFlowTokenRequest {
            guest_token: guest_token.clone(),
            bearer_token: BEARER_TOKEN.to_string(),
            body: data,
        };
        let req = get_flow_token_req.into_request(self.client.clone());
        let res = req.send_request_and_log().await?;
        save_cookies_to_file(&res,cookie_file_path.as_str()).map_err(|e| anyhow!("save cookies failed: {}", e))?;
        let flow: Flow = res.json().await?;
        let flow_token = parse_flow(flow);

        // save cookies
        self.set_guest_token(guest_token);
        self.set_csrf_token(csrf_token);
        Ok(())
    }


    pub async fn is_logged_in(&mut self) -> bool {
        let req = self
            .client
            .get(VERIFY_CREDENTIALS_URL)
            .header("Authorization", format!("Bearer {}", BEARER_TOKEN))
            .header("X-CSRF-Token", self.csrf_token.to_owned())
            .build()
            .unwrap();
        let res = self.client.execute(req).await.unwrap();
        let cookies = res.cookies();
        for cookie in cookies {
            println!("Name: {}, Value: {}", cookie.name(), cookie.value());
            if cookie.name().eq("ct0") {
                self.csrf_token = cookie.value().to_string()
            }
        }
        let text = res.text().await.unwrap();
        let res: VerifyCredentials = serde_json::from_str(&text).unwrap();
        res.errors.is_none()
    }
    pub fn with_cookie_file(cookie_file_path:&str) -> Result<ReAPI, Box<dyn std::error::Error>> {
        // Load the cookies from the file
        // Open the file for reading
        let mut file = File::open(cookie_file_path)?;

        // Read the contents of the file into a string
        let mut cookies_json = String::new();
        file.read_to_string(&mut cookies_json)?;

        // Deserialize the cookies from JSON format
        let cookies: Vec<CookieData> = serde_json::from_str(&cookies_json)?;
        let mut csrf_token = "".to_string();
        // Create a new cookie jar and add the cookies to it
        let cookie_jar = reqwest::cookie::Jar::default();
        println!("cookies len: {}", cookies.len());
        for cookie in cookies {
            // cookie = "foo=bar; Domain=yolo.local";
            if cookie.name.eq("ct0") {
                csrf_token = cookie.clone().value;
            }
            let cookie_str = format!("{}={}; Domain=twitter.com", cookie.name, cookie.value);
            let url = Url::parse("https://twitter.com")?;
            cookie_jar.add_cookie_str(&cookie_str, &url);
        }

        // Create a reqwest client builder
        let cookie_jar_arc = Arc::new(cookie_jar);
        let client_builder = ClientBuilder::new().cookie_provider(cookie_jar_arc);
        // Build the client
        let client = match client_builder.build() {
            Ok(client) => client,
            Err(err) => {
                eprintln!("Error building client: {}", err);
                return Err(err.into());
            }
        };

        Ok(ReAPI {
            client,
            csrf_token: csrf_token.to_string(),
            guest_token: String::from(""),
        })
    }

    // set csrf_token
    pub fn set_csrf_token(&mut self, csrf_token: String) {
        self.csrf_token = csrf_token;
    }

    // set gust_token
    pub fn set_guest_token(&mut self, guest_token: String) {
        self.guest_token = guest_token;
    }

    // set client
    pub fn set_client(&mut self, client: Client) {
        self.client = client;
    }

    fn prepare_request(&self, req: impl IntoRequestBuilder) -> RequestBuilder {
        req.into_request(self.client.clone())
    }

    // get user followers list
    // we dont need to get user fans list becasuse the fans list is so large
    pub async fn get_user_followers_list(
        &self,
        req: GetUserFollowersListRequest,
    ) -> Result<FollowersListResp> {
        let req = req.into_request(self.client.clone());
        let res = req.send_request_and_log().await?;
        let text = res.text().await?;
        let res: FollowersListResp = serde_json::from_str(&text)?;
        Ok(res)
    }

    // get user home page content
    // content contains user profile and user latest tweets
    pub async fn get_user_home_page_content(
        &self,
        req: GetUserHomePageContentRequest,
    ) -> Result<UserHomePageContentResponse> {
        let req = req.into_request(self.client.clone());
        let res = req.send_request_and_log().await?;
        let text = res.text().await?;
        let res: UserHomePageContentResponse = serde_json::from_str(&text)?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_login_in() {
        let mut api = ReAPI::new();
        dotenv::dotenv().ok();
        let name = std::env::var("TWITTER_USER_NAME").unwrap();
        let pwd = std::env::var("TWITTER_USER_PASSWORD").unwrap();
        let req = TwitterLoginRequest {
            user_name: name,
            password: pwd,
            cookie_file_path: Some("ouhuang_cookies.json".to_string()),
        };
        let res = api.login_in(req).await;
    }

    #[tokio::test]
    pub async fn test_login_with_cookies()-> Result<(),anyhow::Error> {
        let mut api = ReAPI::with_cookie_file("ouhuang_cookies.json").map_err(|e| anyhow!("error: {}", e))?;
        let is_logged_in = api.is_logged_in().await;
        assert!(is_logged_in);
        Ok(())
    }

    // get home page
    #[tokio::test]
    async fn test_get_user_home_page_content() {
        let mut api = ReAPI::with_cookie_file("ouhuang_cookies.json").unwrap();
        // https://x.com/xiaomucrypto
        let req = GetUserHomePageContentRequest {
            user_id: "1507631541303713793".to_string(),
            csrf_token: api.csrf_token.clone(),
            bearer_token: BEARER_TOKEN.to_string(),
        };
        let res = api.get_user_home_page_content(req).await.unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_get_user_id_by_screen_name() {
        let api = ReAPI::with_cookie_file("ouhuang_cookies.json").unwrap();

        let req = UserByScreenNameRequest {
            screen_name: "xiaomucrypto".to_string(),
            bearer_token: BEARER_TOKEN.to_string(),
            csrf_token: api.csrf_token.clone(),
        };
        let res = api.get_user_id_by_screen_name(req).await.unwrap();
        println!("{:?}", res);
    }
}
