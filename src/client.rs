
use std::{collections::HashMap, fs::File, io::{Read, Write}, sync::Arc};
use reqwest::{Client, ClientBuilder, RequestBuilder, Response, Url};
use serde_json::json;
use crate::{api::{follow::{twitter_follow_relation_request::GetUserFollowersListRequest, twitter_followers_list_response::FollowersListResp}, homepage::{user_by_screen_name_request::UserByScreenNameRequest, user_by_screen_name_response::UserByScreenNameResponse, user_home_page_content_request::GetUserHomePageContentRequest, user_home_page_content_response::UserHomePageContentResponse}, login::twitter_login::{save_cookies_to_file, CookieData, Flow, GetFlowTokenRequest, GetGuestTokenRequest, TwitterLoginRequest, VerifyCredentials}}, traits::{IntoRequestBuilder, SendRequestAndLog}};
pub use anyhow::anyhow;
pub use anyhow::Result;
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
pub struct XClient  {
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
        // print request cookies
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

impl XClient {
    pub async fn get_guest_id_cookie(&mut self) -> Result<()> {
        let mut form = HashMap::new();
        form.insert("debug", "true");
        form.insert("log", r#"[{"_category_":"client_event","format_version":2,"triggered_on":1722866376530,"items":[{"item_type":0,"id":"1568614885919191041","position":0,"sort_index":"1820458113438842880","impression_details":{"visibility_start":1722866011162,"visibility_end":1722866376529},"first_impression":true,"author_id":"42352056","is_viewer_follows_tweet_author":false,"is_tweet_author_follows_viewer":false,"is_viewer_super_following_tweet_author":false,"is_viewer_super_followed_by_tweet_author":false,"is_tweet_author_super_followable":false,"card_name":"summary_large_image","card_platform":"Web-12","card_url":"https://t.co/wuA9n3gxfG","vanity_url":"guoyu.mirror.xyz","media_details":{"photo_count":0,"content_id":"","publisher_id":"42352056","media_type":-1,"dynamic_ads":false},"media_details_v2":[],"engagement_metrics":{"reply_count":288,"retweet_count":3379,"favorite_count":9387,"quote_count":171}}],"event_namespace":{"page":"profile","section":"tweets","component":"stream","element":"linger","action":"results","client":"m5"},"client_event_sequence_start_timestamp":1722866008056,"client_event_sequence_number":9,"client_app_id":"3033300"}]"#);
        let response = self.client.post("https://api.x.com/1.1/jot/client_event.json")
                .header("Host", "api.x.com")
                .header("pragma", "no-cache")
                .header("cache-control", "no-cache")
                .header("sec-ch-ua", "\"Not)A;Brand\";v=\"99\", \"Google Chrome\";v=\"127\", \"Chromium\";v=\"127\"")
                .header("x-twitter-client-language", "zh-cn")
                .header("sec-ch-ua-mobile", "?0")
                .header("authorization", "Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA")
                .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36")
                .header("content-type", "application/x-www-form-urlencoded")
                .header("x-client-transaction-id", "ra+MSMPaWTdwYhGtBLP2peYl9IJlUjEdBtTUB+WFsg7OzPGSu2Z5Unn78cVcg3VYX/XazK9NJQZzY1R8QHEPXzLGKPtIrg")
                .header("x-twitter-active-user", "yes")
                .header("sec-ch-ua-platform", "\"macOS\"")
                .header("accept", "*/*")
                .header("origin", "https://x.com")
                .header("sec-fetch-site", "same-site")
                .header("sec-fetch-mode", "cors")
                .header("sec-fetch-dest", "empty")
                .header("referer", "https://x.com/")
                .header("accept-language", "zh-CN,zh;q=0.9")
                .header("priority", "u=1, i")
                .form(&form)
                .send()
                .await?;
        // print response status
        println!("Response status: {}", response.status());
        // print cookies
        let cookies = response.cookies();

        let cookie_jar = reqwest::cookie::Jar::default();
        for cookie in cookies {
            // cookie = "foo=bar; Domain=yolo.local";
            let cookie_str = format!("{}={}", cookie.name(), cookie.value());
            let url = Url::parse("https://api.x.com")?;
            cookie_jar.add_cookie_str(&cookie_str, &url);
        }
        // set guest  gt = 1820477076878868982
        let cookie_str = "gt=1820486444403839064";
        let url = Url::parse("https://api.x.com")?;
        cookie_jar.add_cookie_str(&cookie_str, &url);

        // set night_mode	2
        let cookie_str = "night_mode=2";
        let url = Url::parse("https://api.x.com")?;
        cookie_jar.add_cookie_str(&cookie_str, &url);

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
        self.client = client;
        Ok(())
    }

    // get user id by screen name without login
    pub async fn get_user_id_by_screen_name(
        &self,
        req: UserByScreenNameRequest,
    ) -> Result<UserByScreenNameResponse> {
        let req = req.into_request(self.client.clone());
        let res = req.send_request_and_log().await?;
        let text = res.text().await?;
        println!("text: {}", text);
        let res: UserByScreenNameResponse = serde_json::from_str(&text)?;
        Ok(res)
    }

    // user login
    pub async fn login_in(login_req: TwitterLoginRequest) -> Result<XClient, anyhow::Error> {
        let cookie_file_path = login_req
            .cookie_file_path
            .unwrap_or("cookies.json".to_string());
        // get guest token
        let get_guest_token_req = GetGuestTokenRequest {
            bearer_token: BEARER_TOKEN.to_string(),
        };
        let client = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .build()
            .unwrap();
        let req = get_guest_token_req.into_request(client.clone());
        let res = req.send_request_and_log().await?;
        save_cookies_to_file(&res, cookie_file_path.as_str())
            .map_err(|e| anyhow!("save cookies failed: {}", e))?;
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
        let req = get_flow_token_req.into_request(client.clone());
        let res = req.send_request_and_log().await?;
        save_cookies_to_file(&res, cookie_file_path.as_str())
            .map_err(|e| anyhow!("save cookies failed: {}", e))?;
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
        let req = get_flow_token_req.into_request(client.clone());
        let res = req.send_request_and_log().await?;
        save_cookies_to_file(&res, cookie_file_path.as_str())
            .map_err(|e| anyhow!("save cookies failed: {}", e))?;
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
        let req = get_flow_token_req.into_request(client.clone());
        let res = req.send_request_and_log().await?;
        save_cookies_to_file(&res, cookie_file_path.as_str())
            .map_err(|e| anyhow!("save cookies failed: {}", e))?;
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
        let req = get_flow_token_req.into_request(client.clone());
        let res = req.send_request_and_log().await?;
        save_cookies_to_file(&res, cookie_file_path.as_str())
            .map_err(|e| anyhow!("save cookies failed: {}", e))?;
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
        let req = get_flow_token_req.into_request(client.clone());
        let res = req.send_request_and_log().await?;
        save_cookies_to_file(&res, cookie_file_path.as_str())
            .map_err(|e| anyhow!("save cookies failed: {}", e))?;
        let flow: Flow = res.json().await?;
        let flow_token = parse_flow(flow);
        println!("flow_token: {}", flow_token.unwrap());
        // save cookies
        Ok(XClient {
            client,
            csrf_token: csrf_token.to_string(),
            guest_token: String::from(""),
        })
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
    pub fn with_cookie_file(cookie_file_path: &str) -> Result<XClient, Box<dyn std::error::Error>> {
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
        for cookie in cookies {
            // cookie = "foo=bar; Domain=yolo.local";
            if cookie.name.eq("ct0") {
                csrf_token = cookie.clone().value;
            }
            let cookie_str = format!("{}={}; Domain=x.com", cookie.name, cookie.value);
            let url = Url::parse("https://x.com")?;
            cookie_jar.add_cookie_str(&cookie_str, &url);
        }
        let cookie_jar_arc = Arc::new(cookie_jar);
        let client_builder = ClientBuilder::new().cookie_provider(cookie_jar_arc);
        // Build the client
        let client = match client_builder.build() {
            Ok(client) => client,
            Err(err) => {
                panic!("Error building client: {}", err);
            }
        };
        Ok(XClient {
            client,
            csrf_token: csrf_token.to_string(),
            guest_token: String::from(""),
        })
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
        // write to file
        let mut file = File::create("user_home_page_content.json").unwrap();
        file.write_all(text.as_bytes()).unwrap();
        let res: UserHomePageContentResponse = serde_json::from_str(&text)?;
        Ok(res)
    }
}



#[cfg(test)]
mod tests {
    use serde_json::Value;
    use crate::api::{homepage::user_home_page_content_request::GetUserHomePageContentRequest, tweet_details::tweet_detail_view::UserTweetList};

    use super::*;

    #[tokio::test]
    pub async fn test_login_in() -> Result<(), anyhow::Error> {
        dotenv::dotenv().ok();
        let name = std::env::var("TWITTER_USER_NAME").unwrap();
        let pwd = std::env::var("TWITTER_USER_PASSWORD").unwrap();
        let req = TwitterLoginRequest {
            user_name: name,
            password: pwd,
            cookie_file_path: Some("xiaohao1_cookies.json".to_string()),
        };
        let res = XClient::login_in(req).await;
        match res {
            Ok(_) => {
                println!("login success");
            }
            Err(e) => {
                println!("login failed: {}", e);
            }
        }
        Ok(())
    }

    #[tokio::test]
    pub async fn test_login_with_cookies() -> Result<(), anyhow::Error> {
        let mut api = XClient::with_cookie_file("xiaohao1_cookies.json")
            .map_err(|e| anyhow!("error: {}", e))?;
        let is_logged_in = api.is_logged_in().await;
        assert!(is_logged_in);
        Ok(())
    }


    // 根据用户id获取首页 推特内容
    #[tokio::test]
    async fn test_get_user_home_page_content_view() {
        let api = XClient::with_cookie_file("xiaohao1_cookies.json").unwrap();
        let req = GetUserHomePageContentRequest {
            user_id: "1483495485889564674".to_string(),
            csrf_token: api.csrf_token.clone(),
            bearer_token: BEARER_TOKEN.to_string(),
        };
        let res = api.get_user_home_page_content(req).await;
        match res {
            Ok(res) => {
                let tweet_list: UserTweetList = res.try_into().unwrap();
                // pretty json
                println!("{}", serde_json::to_string_pretty(&tweet_list).unwrap());
            }
            Err(e) => {
                println!("出错了: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_user_id_by_screen_name() {
        let api = XClient::with_cookie_file("xiaohao1_cookies.json").unwrap();
        let req = UserByScreenNameRequest {
            screen_name: "bwenews".to_string(),
            bearer_token: BEARER_TOKEN.to_string(),
            csrf_token: api.csrf_token.clone(),
        };
        let res = api.get_user_id_by_screen_name(req).await;
        match res {
            Ok(res) => {
                println!("{:?}", res);
            }
            Err(e) => {
                println!("出错了: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_guest_token() ->Result<()> {
        let client = Client::new();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("authority", "api.twitter.com".parse()?);
        headers.insert("authorization", "Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA".parse()?);
        headers.insert("origin", "https://twitter.com".parse()?);
        headers.insert("referer", "https://twitter.com/".parse()?);
        headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36".parse()?);

        let response: Value = client.post("https://api.twitter.com/1.1/guest/activate.json")
            .headers(headers.clone())
            .send()
            .await?
            .json()
            .await?;

        headers.insert("x-guest-token", response["guest_token"].as_str().unwrap().parse()?);

        let params = [("variables", "{\"screen_name\":\"xiaomucrypto\",\"withSafetyModeUserFields\":true,\"withSuperFollowsUserFields\":true}")];

        let response = client.get("https://twitter.com/i/api/graphql/mCbpQvZAw6zu_4PvuAUVVQ/UserByScreenName")
            .headers(headers)
            .query(&params)
            .send()
            .await?
            .text()
            .await?;

        // print pretty 
        println!("{}", serde_json::to_string_pretty(&response)?);
        Ok(())

    }
}
