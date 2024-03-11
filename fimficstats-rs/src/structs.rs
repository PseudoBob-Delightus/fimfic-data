use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Api {
	data: ApiData,
	included: Vec<ApiIncluded>,
	uri: String,
	method: String,
	debug: ApiDebug,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiData {}

#[derive(Serialize, Deserialize, Debug)]
struct ApiIncluded {
	id: String,
	r#type: String,
	attributes: IncludedAttributes,
	links: IncludedLinks,
	meta: IncludedMeta,
}

#[derive(Serialize, Deserialize, Debug)]
struct IncludedAttributes {
	name: String,
	bio: String,
	bio_html: String,
	num_followers: u32,
	num_stories: u32,
	num_blog_posts: u32,
	avatar: AttributesAvatar,
	color: AttributesColor,
	date_joined: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AttributesAvatar {
	r32: String,
	r48: String,
	r64: String,
	r96: String,
	r128: String,
	r160: String,
	r192: String,
	r256: String,
	r320: String,
	r384: String,
	r512: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AttributesColor {
	hex: String,
	rgb: (u32, u32, u32),
}

#[derive(Serialize, Deserialize, Debug)]
struct IncludedLinks {
	#[serde(rename = "self")]
	link: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IncludedMeta {
	meta: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiDebug {
	duration: String,
}
