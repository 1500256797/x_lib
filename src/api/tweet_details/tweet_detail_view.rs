use crate::api::homepage::user_home_page_content_response::UserHomePageContentResponse;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TweetDetail {
    // 推文ID和用户ID
    pub tweet_id: String,
    pub user_id: String,
    // 创建时间
    pub created_at: String,
    // 推文内容
    pub tweet_content: String,
    // 喜欢、转发、回复和引用数量
    pub like_count: i64,
    pub retweet_count: i64,
    pub reply_count: i64,
    pub bookmark_count: i64,
    // 语言
    pub lang: String,
    // 媒体信息（如图片或视频）的url链接
    pub media: Vec<String>,
    // 引用推文信息
    pub qutoe_tweet_url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
    pub user_id: String,
    pub user_name: String,
    pub user_home_page_url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserTweetList {
    pub user_profile: UserProfile,
    pub tweet_list: Vec<TweetDetail>,
}

#[derive(Debug, Deserialize, thiserror::Error)]
pub enum UserHomePageContentResponseError {
    #[error("UserHomePageContentResponse data is invalid , please check the api")]
    DataError,
}

impl TryFrom<UserHomePageContentResponse> for UserTweetList {
    type Error = UserHomePageContentResponseError;

    fn try_from(value: UserHomePageContentResponse) -> Result<Self, Self::Error> {
        let mut user_profile = UserProfile {
            user_id: "".to_string(),
            user_name: "".to_string(),
            user_home_page_url: "".to_string(),
        };
        let mut tweet_list = vec![];
        let instructions = value
            .data
            .ok_or(UserHomePageContentResponseError::DataError)?
            .user
            .result
            .timeline_v2
            .timeline
            .instructions;

        // get user profile from first instruction
        instructions.iter().for_each(|instruction| {
            if let Some(entries) = &instruction.entries {
                entries.iter().for_each(|entry| {
                    if let Some(content) = entry.content.item_content.as_ref() {
                        let user_profile_data = content
                            .tweet_results
                            .clone()
                            .result
                            .unwrap()
                            .core
                            .user_results
                            .unwrap()
                            .result;
                        user_profile = UserProfile {
                            user_id: user_profile_data.rest_id.clone(),
                            user_name: user_profile_data.legacy.name.clone(),
                            user_home_page_url: format!(
                                "https://twitter.com/{}",
                                user_profile_data.legacy.screen_name.clone()
                            ),
                        };
                    }
                });
            }
        });

        instructions.iter().for_each(|instruction| {
            if let Some(entries) = &instruction.entries {
                entries.iter().for_each(|entry| {
                    if let Some(content) = entry.content.item_content.as_ref() {
                        // let content = entry.content.item_content.as_ref().unwrap();
                        let tweet_result = content.tweet_results.result.as_ref().unwrap();
                        let tweet_detail_legacy = tweet_result.legacy.clone();
                        let tweet_detail = TweetDetail {
                            tweet_id: tweet_detail_legacy.id_str,
                            user_id: tweet_detail_legacy.user_id_str,
                            created_at: tweet_detail_legacy.created_at.clone(),
                            tweet_content: tweet_detail_legacy.full_text,
                            like_count: tweet_detail_legacy.favorite_count,
                            retweet_count: tweet_detail_legacy.retweet_count,
                            reply_count: tweet_detail_legacy.reply_count,
                            bookmark_count: tweet_detail_legacy.bookmark_count,
                            lang: tweet_detail_legacy.lang.clone(),
                            media: vec![],
                            qutoe_tweet_url: tweet_detail_legacy
                                .quoted_status_permalink
                                .as_ref()
                                .map(|x| x.expanded.clone()),
                        };
                        tweet_list.push(tweet_detail);
                    }
                });
            }
        });

        Ok(UserTweetList {
            user_profile,
            tweet_list,
        })
    }
}
