use crate::api::homepage::user_home_page_content_request::GetUserHomePageContentRequest;
use crate::traits::IntoRequestBuilder;

pub struct GetUserTweetDetailsRequest {
    pub guest_token: String,
    pub csrf_token: String,
    pub tweet_id: String,
}
