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
    pub permalink: String,
    #[serde(rename = "parent_whitelist_status")]
    pub url: String,
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
