use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Api {
	pub data: ApiData,
	pub included: Vec<ApiIncluded>,
	pub uri: String,
	pub method: String,
	pub debug: ApiDebug,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiData {
	pub id: String,
	pub r#type: String,
	pub attributes: DataAttributes,
	pub relationships: DataRelationships,
	pub links: DataLinks,
	pub meta: DataMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataAttributes {
	pub title: String,
	pub short_description: String,
	pub description: String,
	pub description_html: String,
	pub date_modified: String,
	pub date_updated: String,
	pub date_published: String,
	pub published: bool,
	pub cover_image: Option<AttributesCoverImage>,
	pub color: AttributesColor,
	pub num_views: u32,
	pub total_num_views: u32,
	pub num_words: u32,
	pub num_chapters: u32,
	pub num_comments: u32,
	pub rating: u32,
	pub status: String,
	pub submitted: bool,
	pub completion_status: String,
	pub content_rating: String,
	pub num_likes: i32,
	pub num_dislikes: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AttributesCoverImage {
	pub thumbnail: String,
	pub medium: String,
	pub large: String,
	pub full: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataRelationships {
	pub author: RelationshipAuthor,
	pub tags: RelationshipTags,
	pub prequel: Option<RelationshipPrequel>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelationshipAuthor {
	pub data: AuthorData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthorData {
	pub r#type: String,
	pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelationshipTags {
	pub data: Vec<TagData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TagData {
	pub r#type: String,
	pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelationshipPrequel {
	pub data: PrequelData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PrequelData {
	pub r#type: String,
	pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataLinks {
	#[serde(rename = "self")]
	pub link: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataMeta {
	pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiIncluded {
	pub id: String,
	pub r#type: String,
	pub attributes: IncludedAttributes,
	pub links: IncludedLinks,
	pub meta: IncludedMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IncludedAttributes {
	pub name: String,
	pub bio: String,
	pub bio_html: String,
	pub num_followers: u32,
	pub num_stories: u32,
	pub num_blog_posts: u32,
	pub avatar: AttributesAvatar,
	pub color: AttributesColor,
	pub date_joined: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AttributesAvatar {
	#[serde(rename = "32")]
	pub r32: String,
	#[serde(rename = "48")]
	pub r48: String,
	#[serde(rename = "64")]
	pub r64: String,
	#[serde(rename = "96")]
	pub r96: String,
	#[serde(rename = "128")]
	pub r128: String,
	#[serde(rename = "160")]
	pub r160: String,
	#[serde(rename = "192")]
	pub r192: String,
	#[serde(rename = "256")]
	pub r256: String,
	#[serde(rename = "320")]
	pub r320: String,
	#[serde(rename = "384")]
	pub r384: String,
	#[serde(rename = "512")]
	pub r512: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AttributesColor {
	pub hex: String,
	pub rgb: (u32, u32, u32),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IncludedLinks {
	#[serde(rename = "self")]
	pub link: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IncludedMeta {
	pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiDebug {
	pub duration: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stats {
	pub chapters: Vec<ChapterStats>,
	pub stats: StatsStats,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChapterStats {
	pub date: String,
	pub title: String,
	pub views: String,
	pub words: String,
	pub words_text: String,
	pub chapter_num: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StatsStats {
	pub data: Vec<StatsData>,
	pub first_chapter_date: ChapterDate,
	pub last_chapter_date: ChapterDate,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StatsData {
	pub views: Option<u32>,
	pub likes: Option<u32>,
	pub dislikes: Option<u32>,
	pub date: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ChapterDate {
	Text(String),
	Number(u32),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeckodriverSession {
	pub value: SessionValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionValue {
	#[serde(rename = "sessionId")]
	pub session_id: String,
	pub capabilities: Capabilities,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Capabilities {
	#[serde(rename = "acceptInsecureCerts")]
	pub accept_insecure_certs: bool,
	#[serde(rename = "browserName")]
	pub browser_name: String,
	#[serde(rename = "browserVersion")]
	pub browser_version: String,
	#[serde(rename = "moz:accessibilityChecks")]
	pub moz_accessibility_checks: bool,
	#[serde(rename = "moz:buildID")]
	pub moz_build_id: String,
	#[serde(rename = "moz:geckodriverVersion")]
	pub moz_geckodriver_version: String,
	#[serde(rename = "moz:headless")]
	pub moz_headless: bool,
	#[serde(rename = "moz:platformVersion")]
	pub moz_platform_version: String,
	#[serde(rename = "moz:processID")]
	pub moz_process_id: u32,
	#[serde(rename = "moz:profile")]
	pub moz_profile: String,
	#[serde(rename = "moz:shutdownTimeout")]
	pub moz_shutdown_timeout: u64,
	#[serde(rename = "moz:webdriverClick")]
	pub moz_webdriver_click: bool,
	#[serde(rename = "moz:windowless")]
	pub moz_windowless: bool,
	#[serde(rename = "pageLoadStrategy")]
	pub page_load_strategy: String,
	#[serde(rename = "platformName")]
	pub platform_name: String,
	pub proxy: Proxy,
	#[serde(rename = "setWindowRect")]
	pub set_window_rect: bool,
	#[serde(rename = "strictFileInteractability")]
	pub strict_file_interactability: bool,
	pub timeouts: Timeouts,
	#[serde(rename = "unhandledPromptBehavior")]
	pub unhandled_prompt_behavior: String,
	#[serde(rename = "userAgent")]
	pub user_agent: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Proxy {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timeouts {
	pub implicit: u64,
	#[serde(rename = "pageLoad")]
	pub page_load: u64,
	pub script: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriverSetting {
	pub value: Option<bool>,
}
