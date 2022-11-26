use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedditResponseRoot {
    pub kind: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub after: String,
    pub dist: i64,
    pub modhash: String,
    #[serde(rename = "geo_filter")]
    pub geo_filter: String,
    pub children: Vec<Children>,
    pub before: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children {
    pub kind: String,
    pub data: Data2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data2 {
    #[serde(rename = "approved_at_utc")]
    pub approved_at_utc: Value,
    pub subreddit: String,
    pub selftext: String,
    #[serde(rename = "author_fullname")]
    pub author_fullname: String,
    pub saved: bool,
    #[serde(rename = "mod_reason_title")]
    pub mod_reason_title: Value,
    pub gilded: i64,
    pub clicked: bool,
    #[serde(rename = "is_gallery")]
    pub is_gallery: Option<bool>,
    pub title: String,
    #[serde(rename = "link_flair_richtext")]
    pub link_flair_richtext: Vec<Value>,
    #[serde(rename = "subreddit_name_prefixed")]
    pub subreddit_name_prefixed: String,
    pub hidden: bool,
    pub pwls: i64,
    #[serde(rename = "link_flair_css_class")]
    pub link_flair_css_class: Value,
    pub downs: i64,
    #[serde(rename = "thumbnail_height")]
    pub thumbnail_height: Option<i64>,
    #[serde(rename = "top_awarded_type")]
    pub top_awarded_type: Value,
    #[serde(rename = "hide_score")]
    pub hide_score: bool,
    pub quarantine: bool,
    #[serde(rename = "link_flair_text_color")]
    pub link_flair_text_color: String,
    #[serde(rename = "upvote_ratio")]
    pub upvote_ratio: f64,
    #[serde(rename = "author_flair_background_color")]
    pub author_flair_background_color: Value,
    pub ups: i64,
    pub domain: String,
    #[serde(rename = "media_embed")]
    pub media_embed: MediaEmbed,
    #[serde(rename = "thumbnail_width")]
    pub thumbnail_width: Option<i64>,
    #[serde(rename = "author_flair_template_id")]
    pub author_flair_template_id: Value,
    #[serde(rename = "is_original_content")]
    pub is_original_content: bool,
    #[serde(rename = "user_reports")]
    pub user_reports: Vec<Value>,
    #[serde(rename = "secure_media")]
    pub secure_media: Value,
    #[serde(rename = "is_reddit_media_domain")]
    pub is_reddit_media_domain: bool,
    #[serde(rename = "is_meta")]
    pub is_meta: bool,
    pub category: Value,
    #[serde(rename = "secure_media_embed")]
    pub secure_media_embed: SecureMediaEmbed,
    #[serde(rename = "gallery_data")]
    pub gallery_data: Option<GalleryData>,
    #[serde(rename = "link_flair_text")]
    pub link_flair_text: Value,
    #[serde(rename = "can_mod_post")]
    pub can_mod_post: bool,
    pub score: i64,
    #[serde(rename = "approved_by")]
    pub approved_by: Value,
    #[serde(rename = "is_created_from_ads_ui")]
    pub is_created_from_ads_ui: bool,
    #[serde(rename = "author_premium")]
    pub author_premium: bool,
    pub thumbnail: String,
    pub edited: bool,
    #[serde(rename = "author_flair_css_class")]
    pub author_flair_css_class: Value,
    #[serde(rename = "author_flair_richtext")]
    pub author_flair_richtext: Vec<Value>,
    pub gildings: Gildings,
    #[serde(rename = "content_categories")]
    pub content_categories: Value,
    #[serde(rename = "is_self")]
    pub is_self: bool,
    #[serde(rename = "subreddit_type")]
    pub subreddit_type: String,
    pub created: f64,
    #[serde(rename = "link_flair_type")]
    pub link_flair_type: String,
    pub wls: i64,
    #[serde(rename = "removed_by_category")]
    pub removed_by_category: Value,
    #[serde(rename = "banned_by")]
    pub banned_by: Value,
    #[serde(rename = "author_flair_type")]
    pub author_flair_type: String,
    #[serde(rename = "total_awards_received")]
    pub total_awards_received: i64,
    #[serde(rename = "allow_live_comments")]
    pub allow_live_comments: bool,
    #[serde(rename = "selftext_html")]
    pub selftext_html: Option<String>,
    pub likes: Value,
    #[serde(rename = "suggested_sort")]
    pub suggested_sort: Value,
    #[serde(rename = "banned_at_utc")]
    pub banned_at_utc: Value,
    #[serde(rename = "url_overridden_by_dest")]
    pub url_overridden_by_dest: Option<String>,
    #[serde(rename = "view_count")]
    pub view_count: Value,
    pub archived: bool,
    #[serde(rename = "no_follow")]
    pub no_follow: bool,
    #[serde(rename = "is_crosspostable")]
    pub is_crosspostable: bool,
    pub pinned: bool,
    #[serde(rename = "over_18")]
    pub over_18: bool,
    #[serde(rename = "all_awardings")]
    pub all_awardings: Vec<AllAwarding>,
    pub awarders: Vec<Value>,
    #[serde(rename = "media_only")]
    pub media_only: bool,
    #[serde(rename = "can_gild")]
    pub can_gild: bool,
    pub spoiler: bool,
    pub locked: bool,
    #[serde(rename = "author_flair_text")]
    pub author_flair_text: Value,
    #[serde(rename = "treatment_tags")]
    pub treatment_tags: Vec<Value>,
    pub visited: bool,
    #[serde(rename = "removed_by")]
    pub removed_by: Value,
    #[serde(rename = "mod_note")]
    pub mod_note: Value,
    pub distinguished: Value,
    #[serde(rename = "subreddit_id")]
    pub subreddit_id: String,
    #[serde(rename = "author_is_blocked")]
    pub author_is_blocked: bool,
    #[serde(rename = "mod_reason_by")]
    pub mod_reason_by: Value,
    #[serde(rename = "num_reports")]
    pub num_reports: Value,
    #[serde(rename = "removal_reason")]
    pub removal_reason: Value,
    #[serde(rename = "link_flair_background_color")]
    pub link_flair_background_color: String,
    pub id: String,
    #[serde(rename = "is_robot_indexable")]
    pub is_robot_indexable: bool,
    #[serde(rename = "report_reasons")]
    pub report_reasons: Value,
    pub author: String,
    #[serde(rename = "discussion_type")]
    pub discussion_type: Value,
    #[serde(rename = "num_comments")]
    pub num_comments: i64,
    #[serde(rename = "send_replies")]
    pub send_replies: bool,
    #[serde(rename = "whitelist_status")]
    pub whitelist_status: String,
    #[serde(rename = "contest_mode")]
    pub contest_mode: bool,
    #[serde(rename = "mod_reports")]
    pub mod_reports: Vec<Value>,
    #[serde(rename = "author_patreon_flair")]
    pub author_patreon_flair: bool,
    #[serde(rename = "author_flair_text_color")]
    pub author_flair_text_color: Value,
    pub permalink: String,
    #[serde(rename = "parent_whitelist_status")]
    pub parent_whitelist_status: String,
    pub stickied: bool,
    pub url: String,
    #[serde(rename = "subreddit_subscribers")]
    pub subreddit_subscribers: i64,
    #[serde(rename = "created_utc")]
    pub created_utc: f64,
    #[serde(rename = "num_crossposts")]
    pub num_crossposts: i64,
    pub media: Value,
    #[serde(rename = "is_video")]
    pub is_video: bool,
    #[serde(rename = "post_hint")]
    pub post_hint: Option<String>,
    pub preview: Option<Preview>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaEmbed {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureMediaEmbed {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GalleryData {
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "media_id")]
    pub media_id: String,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gildings {
    #[serde(rename = "gid_1")]
    pub gid_1: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllAwarding {
    #[serde(rename = "giver_coin_reward")]
    pub giver_coin_reward: Value,
    #[serde(rename = "subreddit_id")]
    pub subreddit_id: Value,
    #[serde(rename = "is_new")]
    pub is_new: bool,
    #[serde(rename = "days_of_drip_extension")]
    pub days_of_drip_extension: Value,
    #[serde(rename = "coin_price")]
    pub coin_price: i64,
    pub id: String,
    #[serde(rename = "penny_donate")]
    pub penny_donate: Value,
    #[serde(rename = "award_sub_type")]
    pub award_sub_type: String,
    #[serde(rename = "coin_reward")]
    pub coin_reward: i64,
    #[serde(rename = "icon_url")]
    pub icon_url: String,
    #[serde(rename = "days_of_premium")]
    pub days_of_premium: Value,
    #[serde(rename = "tiers_by_required_awardings")]
    pub tiers_by_required_awardings: Value,
    #[serde(rename = "resized_icons")]
    pub resized_icons: Vec<ResizedIcon>,
    #[serde(rename = "icon_width")]
    pub icon_width: i64,
    #[serde(rename = "static_icon_width")]
    pub static_icon_width: i64,
    #[serde(rename = "start_date")]
    pub start_date: Value,
    #[serde(rename = "is_enabled")]
    pub is_enabled: bool,
    #[serde(rename = "awardings_required_to_grant_benefits")]
    pub awardings_required_to_grant_benefits: Value,
    pub description: String,
    #[serde(rename = "end_date")]
    pub end_date: Value,
    #[serde(rename = "sticky_duration_seconds")]
    pub sticky_duration_seconds: Value,
    #[serde(rename = "subreddit_coin_reward")]
    pub subreddit_coin_reward: i64,
    pub count: i64,
    #[serde(rename = "static_icon_height")]
    pub static_icon_height: i64,
    pub name: String,
    #[serde(rename = "resized_static_icons")]
    pub resized_static_icons: Vec<ResizedStaticIcon>,
    #[serde(rename = "icon_format")]
    pub icon_format: Value,
    #[serde(rename = "icon_height")]
    pub icon_height: i64,
    #[serde(rename = "penny_price")]
    pub penny_price: Value,
    #[serde(rename = "award_type")]
    pub award_type: String,
    #[serde(rename = "static_icon_url")]
    pub static_icon_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResizedIcon {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResizedStaticIcon {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    pub images: Vec<RedditImage>,
    pub enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedditImage {
    pub source: Source,
    pub resolutions: Vec<Resolution>,
    pub variants: Variants,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variants {
    pub gif: Option<Gif>,
    pub mp4: Option<Mp4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gif {
    pub source: Source2,
    pub resolutions: Vec<Resolution2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source2 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution2 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mp4 {
    pub source: Source3,
    pub resolutions: Vec<Resolution3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source3 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution3 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}
