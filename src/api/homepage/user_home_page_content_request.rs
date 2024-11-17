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
            {"rweb_tipjar_consumption_enabled":true,"responsive_web_graphql_exclude_directive_enabled":true,"verified_phone_label_enabled":false,"creator_subscriptions_tweet_preview_api_enabled":true,"responsive_web_graphql_timeline_navigation_enabled":true,"responsive_web_graphql_skip_user_profile_image_extensions_enabled":false,"communities_web_enable_tweet_community_results_fetch":true,"c9s_tweet_anatomy_moderator_badge_enabled":true,"articles_preview_enabled":true,"responsive_web_edit_tweet_api_enabled":true,"graphql_is_translatable_rweb_tweet_is_translatable_enabled":true,"view_counts_everywhere_api_enabled":true,"longform_notetweets_consumption_enabled":true,"responsive_web_twitter_article_tweet_consumption_enabled":true,"tweet_awards_web_tipping_enabled":false,"creator_subscriptions_quote_tweet_preview_enabled":false,"freedom_of_speech_not_reach_fetch_enabled":true,"standardized_nudges_misinfo":true,"tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled":true,"rweb_video_timestamps_enabled":true,"longform_notetweets_rich_text_read_enabled":true,"longform_notetweets_inline_media_enabled":true,"responsive_web_enhance_cards_enabled":false}
        );
        let field_troggles = json!(
            {"withArticlePlainText":false}
        );
        let query_param = [
            ("variables", variables.to_string()),
            ("features", features.to_string()),
            ("fieldTroggles", field_troggles.to_string()),
        ];

        client
            .get("https://x.com/i/api/graphql/Tg82Ez_kxVaJf7OPbUdbCg/UserTweets")
            .header("Authorization", format!("Bearer {}", self.bearer_token))
            .header("X-CSRF-Token", self.csrf_token.to_owned())
            .query(&query_param)
    }
}
