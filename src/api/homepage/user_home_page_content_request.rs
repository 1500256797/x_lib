use crate::traits::IntoRequestBuilder;
use serde_json::json;

pub struct GetUserHomePageContentRequest {
    pub user_id: String,
    pub csrf_token: String,
    pub bearer_token: String,
}

impl IntoRequestBuilder for GetUserHomePageContentRequest {
    fn into_request(self, client: reqwest::Client) -> reqwest::RequestBuilder {
        let variables = json!(
            {"userId":self.user_id,"count":20,"includePromotedContent":true,"withQuickPromoteEligibilityTweetFields":true,"withVoice":true,"withV2Timeline":true}
        );
        let features = json!(
            {"responsive_web_graphql_exclude_directive_enabled":true,"verified_phone_label_enabled":false,"responsive_web_home_pinned_timelines_enabled":true,"creator_subscriptions_tweet_preview_api_enabled":true,"responsive_web_graphql_timeline_navigation_enabled":true,"responsive_web_graphql_skip_user_profile_image_extensions_enabled":false,"c9s_tweet_anatomy_moderator_badge_enabled":true,"tweetypie_unmention_optimization_enabled":true,"responsive_web_edit_tweet_api_enabled":true,"graphql_is_translatable_rweb_tweet_is_translatable_enabled":true,"view_counts_everywhere_api_enabled":true,"longform_notetweets_consumption_enabled":true,"responsive_web_twitter_article_tweet_consumption_enabled":false,"tweet_awards_web_tipping_enabled":false,"freedom_of_speech_not_reach_fetch_enabled":true,"standardized_nudges_misinfo":true,"tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled":true,"longform_notetweets_rich_text_read_enabled":true,"longform_notetweets_inline_media_enabled":true,"responsive_web_media_download_video_enabled":false,"responsive_web_enhance_cards_enabled":false}
        );
        let query_param = [
            ("variables", variables.to_string()),
            ("features", features.to_string()),
        ];

        client
            .get("https://twitter.com/i/api/graphql/VgitpdpNZ-RUIp5D1Z_D-A/UserTweets")
            .header("Authorization", format!("Bearer {}", self.bearer_token))
            .header("X-CSRF-Token", self.csrf_token.to_owned())
            .query(&query_param)
    }
}
