use serde_json::json;
use crate::traits::IntoRequestBuilder;

pub struct UserByScreenNameRequest {
    pub screen_name: String,
    pub bearer_token: String,
    pub csrf_token: String,
}


impl IntoRequestBuilder for UserByScreenNameRequest {
    fn into_request(self, client: reqwest::Client) -> reqwest::RequestBuilder {
        let variables = json!({"screen_name":self.screen_name,"withSafetyModeUserFields":true});
        let features = json!(
        {"hidden_profile_subscriptions_enabled":true,"rweb_tipjar_consumption_enabled":true,"responsive_web_graphql_exclude_directive_enabled":true,"verified_phone_label_enabled":false,"subscriptions_verification_info_is_identity_verified_enabled":true,"subscriptions_verification_info_verified_since_enabled":true,"highlights_tweets_tab_ui_enabled":true,"responsive_web_twitter_article_notes_tab_enabled":true,"subscriptions_feature_can_gift_premium":true,"creator_subscriptions_tweet_preview_api_enabled":true,"responsive_web_graphql_skip_user_profile_image_extensions_enabled":false,"responsive_web_graphql_timeline_navigation_enabled":true});

        let field_toggles = json! ({"withAuxiliaryUserLabels":false});
        let query_param = [
            ("variables", variables.to_string()),
            ("features", features.to_string()),
            ("fieldToggles", field_toggles.to_string()),
        ];
        
        client
            .get("https://api.x.com/graphql/Yka-W8dz7RaEuQNkroPkYw/UserByScreenName")
            .header("Authorization", format!("Bearer {}", self.bearer_token))
            .header("X-CSRF-Token", self.csrf_token.to_owned())
            .query(&query_param)
            
    }
}