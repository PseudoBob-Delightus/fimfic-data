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
struct ApiData {
	id: String,
	r#type: String,
	attributes: DataAttributes,
	relationships: DataRelationships,
	links: DataLinks,
	meta: DataMeta,
}

#[derive(Serialize, Deserialize, Debug)]
struct DataAttributes {
	title: String,
	short_description: String,
	description: String,
	description_html: String,
	date_modified: String,
	date_updated: String,
	date_published: String,
	published: bool,
	cover_image: Option<AttributesCoverImage>,
	color: AttributesColor,
	num_views: u32,
	total_num_views: u32,
	num_words: u32,
	num_chapters: u32,
	num_comments: u32,
	rating: u32,
	status: String,
	submitted: bool,
	completion_status: String,
	content_rating: String,
	num_likes: i32,
	num_dislikes: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct AttributesCoverImage {
	thumbnail: String,
	medium: String,
	large: String,
	full: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DataRelationships {
	author: RelationshipAuthor,
	tags: RelationshipTags,
	prequel: Option<RelationshipPrequel>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RelationshipAuthor {
	data: AuthorData,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthorData {
	r#type: String,
	id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RelationshipTags {
	data: Vec<TagData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TagData {
	r#type: String,
	id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RelationshipPrequel {
	data: PrequelData,
}

#[derive(Serialize, Deserialize, Debug)]
struct PrequelData {
	r#type: String,
	id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DataLinks {
	#[serde(rename = "self")]
	link: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DataMeta {
	meta: String,
}

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
