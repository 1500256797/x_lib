use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserByScreenNameResponse {
    pub errors: Vec<Error>,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub message: String,
    pub locations: Vec<Location>,
    pub path: Vec<String>,
    pub extensions: Extensions,
    pub code: i64,
    pub kind: String,
    pub name: String,
    pub source: String,
    pub tracing: Tracing2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub line: i64,
    pub column: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    pub name: String,
    pub source: String,
    pub code: i64,
    pub kind: String,
    pub tracing: Tracing,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracing {
    #[serde(rename = "trace_id")]
    pub trace_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracing2 {
    #[serde(rename = "trace_id")]
    pub trace_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub result: Result,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub id: String,
    #[serde(rename = "rest_id")]
    pub rest_id: String,
    #[serde(rename = "affiliates_highlighted_label")]
    pub affiliates_highlighted_label: AffiliatesHighlightedLabel,
    #[serde(rename = "is_blue_verified")]
    pub is_blue_verified: bool,
    #[serde(rename = "profile_image_shape")]
    pub profile_image_shape: String,
    pub legacy: Legacy,
    pub professional: Professional,
    #[serde(rename = "tipjar_settings")]
    pub tipjar_settings: TipjarSettings,
    #[serde(rename = "legacy_extended_profile")]
    pub legacy_extended_profile: LegacyExtendedProfile,
    #[serde(rename = "is_profile_translatable")]
    pub is_profile_translatable: bool,
    #[serde(rename = "has_hidden_subscriptions_on_profile")]
    pub has_hidden_subscriptions_on_profile: bool,
    #[serde(rename = "verification_info")]
    pub verification_info: VerificationInfo,
    #[serde(rename = "highlights_info")]
    pub highlights_info: HighlightsInfo,
    #[serde(rename = "user_seed_tweet_count")]
    pub user_seed_tweet_count: i64,
    #[serde(rename = "business_account")]
    pub business_account: BusinessAccount,
    #[serde(rename = "creator_subscriptions_count")]
    pub creator_subscriptions_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliatesHighlightedLabel {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Legacy {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "default_profile")]
    pub default_profile: bool,
    #[serde(rename = "default_profile_image")]
    pub default_profile_image: bool,
    pub description: String,
    pub entities: Entities,
    #[serde(rename = "fast_followers_count")]
    pub fast_followers_count: i64,
    #[serde(rename = "favourites_count")]
    pub favourites_count: i64,
    #[serde(rename = "followers_count")]
    pub followers_count: i64,
    #[serde(rename = "friends_count")]
    pub friends_count: i64,
    #[serde(rename = "has_custom_timelines")]
    pub has_custom_timelines: bool,
    #[serde(rename = "is_translator")]
    pub is_translator: bool,
    #[serde(rename = "listed_count")]
    pub listed_count: i64,
    pub location: String,
    #[serde(rename = "media_count")]
    pub media_count: i64,
    pub name: String,
    #[serde(rename = "normal_followers_count")]
    pub normal_followers_count: i64,
    #[serde(rename = "pinned_tweet_ids_str")]
    pub pinned_tweet_ids_str: Vec<String>,
    #[serde(rename = "possibly_sensitive")]
    pub possibly_sensitive: bool,
    #[serde(rename = "profile_banner_url")]
    pub profile_banner_url: String,
    #[serde(rename = "profile_image_url_https")]
    pub profile_image_url_https: String,
    #[serde(rename = "profile_interstitial_type")]
    pub profile_interstitial_type: String,
    #[serde(rename = "screen_name")]
    pub screen_name: String,
    #[serde(rename = "statuses_count")]
    pub statuses_count: i64,
    #[serde(rename = "translator_type")]
    pub translator_type: String,
    pub verified: bool,
    #[serde(rename = "withheld_in_countries")]
    pub withheld_in_countries: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entities {
    pub description: Description,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub urls: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Professional {
    #[serde(rename = "rest_id")]
    pub rest_id: String,
    #[serde(rename = "professional_type")]
    pub professional_type: String,
    pub category: Vec<Category>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i64,
    pub name: String,
    #[serde(rename = "icon_name")]
    pub icon_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TipjarSettings {
    #[serde(rename = "is_enabled")]
    pub is_enabled: bool,
    #[serde(rename = "bandcamp_handle")]
    pub bandcamp_handle: String,
    #[serde(rename = "bitcoin_handle")]
    pub bitcoin_handle: String,
    #[serde(rename = "cash_app_handle")]
    pub cash_app_handle: String,
    #[serde(rename = "ethereum_handle")]
    pub ethereum_handle: String,
    #[serde(rename = "gofundme_handle")]
    pub gofundme_handle: String,
    #[serde(rename = "patreon_handle")]
    pub patreon_handle: String,
    #[serde(rename = "pay_pal_handle")]
    pub pay_pal_handle: String,
    #[serde(rename = "venmo_handle")]
    pub venmo_handle: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyExtendedProfile {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationInfo {
    #[serde(rename = "is_identity_verified")]
    pub is_identity_verified: bool,
    pub reason: Reason,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reason {
    pub description: Description2,
    #[serde(rename = "verified_since_msec")]
    pub verified_since_msec: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description2 {
    pub text: String,
    pub entities: Vec<Entity>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    #[serde(rename = "from_index")]
    pub from_index: i64,
    #[serde(rename = "to_index")]
    pub to_index: i64,
    #[serde(rename = "ref")]
    pub ref_field: Ref,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ref {
    pub url: String,
    #[serde(rename = "url_type")]
    pub url_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HighlightsInfo {
    #[serde(rename = "can_highlight_tweets")]
    pub can_highlight_tweets: bool,
    #[serde(rename = "highlighted_tweets")]
    pub highlighted_tweets: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessAccount {}
