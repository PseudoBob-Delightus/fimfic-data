use chrono::Utc;
use pony::averages::SimpleMovingAverage;
use pony::time::format_milliseconds;
use rusqlite::Connection;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;

#[derive(Debug, Clone)]
struct StatsPage {
	tags: Vec<StoryTag>,
	stats: Stats,
	ranking: u32,
	word_ranking: u32,
	bookshelves: u32,
	tracking: u32,
	referrals: HashMap<String, u32>,
	page_time: String,
}

#[derive(Debug, Clone)]
struct StoryTag {
	id: u32,
	title: String,
	group: String,
	href: String,
	text: String,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	println!("Program started at: {}", Utc::now());
	let program_start = unix_time()?;

	let version = 1.0;

	let mut story_map = HashMap::new();

	let file = File::open("/home/velvetremedy/fimfic/fimfiction-stats/manifest.tsv")?;
	let reader = BufReader::new(file);
	for line in reader.lines().skip(1) {
		let line = line?;
		let parts = line.split('\t').collect::<Vec<_>>();
		let url = parts[1];
		let timestamp = parts[2];
		let http_status = parts[3];
		let status = http_status.parse::<u32>()?;
		if status != 200 {
			continue;
		}
		if !url.contains("fimfiction") {
			continue;
		}
		let timestamp = timestamp.parse::<u32>()?;
		let story_id = url.split('/').next_back().unwrap().parse::<u32>()?;
		story_map.insert(story_id, timestamp);
	}

	let mut db = setup_database()?;

	// Simple weighted average times, used for estimating runtime.
	let mut times = SimpleMovingAverage::<u32>::new(10_000);

	let total = story_map.len();

	// Loop over IDs to scrape data.
	for (i, (id, timestamp)) in story_map.iter().enumerate() {
		let start_time = unix_time()?;

		let path = format!(
			"/home/velvetremedy/fimfic/fimfiction-stats/www.fimfiction.net/story/stats/{id}.html"
		);
		let html = fs::read_to_string(path).await?;
		let stats = parse_stats_page(html);

		if !times.data.is_empty() {
			let average = times.average().unwrap();
			println!(
				"Iteration: {i:6}, ID: {id:6}, estimated time remaining: {}",
				format_milliseconds((average * (total - i) as u32) as u128, Some(3))?
			);
		}

		let end_time = unix_time()?;
		times.insert((end_time - start_time) as u32);
	}

	println!("Total stories: {total}");

	let program_end = unix_time()?;
	let time = format_milliseconds(program_end - program_start, None)?;
	println!("Total runtime: {time}");
	println!("Program ended at: {}", Utc::now());
	Ok(())
}

fn unix_time() -> Result<u128, Box<dyn Error>> {
	Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())
}

fn get_attribute_if(html: &Html, condition: &str, attribute: Option<&str>) -> Option<String> {
	let selector = Selector::parse(condition).unwrap();
	if let Some(element) = html.select(&selector).next() {
		if let Some(attribute) = attribute {
			element.value().attr(attribute).map(|link| link.into())
		} else {
			Some(element.text().collect())
		}
	} else {
		None
	}
}

fn get_attributes_from(
	html: &Html, from: &str, attribute: Option<&str>, capacity: usize,
) -> Vec<String> {
	let mut attributes = Vec::with_capacity(capacity);
	let selector = Selector::parse(from).unwrap();
	for child in html.select(&selector) {
		if let Some(attribute) = attribute {
			if let Some(text) = child.value().attr(attribute) {
				attributes.push(text.into())
			}
		} else {
			attributes.push(child.text().collect())
		}
	}
	attributes
}

fn parse_stats_page(html: String) -> StatsPage {
	let html = Html::parse_document(&html);

	let published =
		get_attributes_from_parent(&html, ".story-page-header .mini-info-box li > b", None, 1);

	let parts: Vec<_> = published[0].split_whitespace().collect();
	let day = parts[1].trim_end_matches(|c: char| !c.is_ascii_digit());
	let month = parse_month(parts[2]);
	let year = parts[3];
	let published = format!("{year}-{month}-{day}");

	let short_desc = get_attributes_from(&html, ".story-page-header .desktop p", None, 1);
	let short_desc = short_desc.first().unwrap().to_owned();

	let tags = get_story_tags(&html);

	let stats = get_attribute_if(&html, ".layout-two-columns.story-stats", Some("data-data"));
	let stats = serde_json::from_str::<Stats>(&stats.unwrap()).unwrap();

	let sidebar_stats =
		get_attributes_from_parent(&html, ".content_box .article ul li > b", None, 6);
	let ranking = sidebar_stats.get(2).map(|s| get_right_stat(s)).unwrap();

	let word_ranking = sidebar_stats.get(3).map(|s| get_right_stat(s)).unwrap();

	let bookshelves = sidebar_stats.get(4).map(|s| get_right_stat(s)).unwrap();

	let tracking = sidebar_stats.get(5).map(|s| get_right_stat(s)).unwrap();

	let referrals: HashMap<String, u32> = sidebar_stats
		.iter()
		.skip(6)
		.map(|referral| {
			let data = referral.split(": ").collect::<Vec<_>>();
			let site = data.first().unwrap();
			let count = get_right_stat(data.last().unwrap());
			(site.to_string(), count)
		})
		.collect();

	let page_time = get_page_time(&html);

	StatsPage {
		tags,
		stats,
		ranking,
		word_ranking,
		bookshelves,
		tracking,
		referrals,
		page_time,
	}
}

fn get_attributes_from_parent(
	html: &Html, from: &str, attribute: Option<&str>, capacity: usize,
) -> Vec<String> {
	let mut attributes = Vec::with_capacity(capacity);
	let selector = Selector::parse(from).unwrap();
	for child in html.select(&selector) {
		if let Some(parent_node) = child.parent()
			&& let Some(parent_element) = ElementRef::wrap(parent_node)
		{
			if let Some(attribute) = attribute {
				if let Some(text) = parent_element.value().attr(attribute) {
					attributes.push(text.into())
				}
			} else {
				attributes.push(parent_element.text().collect())
			}
		}
	}
	attributes
}

fn get_right_stat(text: &str) -> u32 {
	text.split(':')
		.next_back()
		.unwrap()
		.chars()
		.filter(|c| c.is_ascii_digit())
		.collect::<String>()
		.parse::<u32>()
		.unwrap()
}

fn get_page_time(html: &Html) -> String {
	let page_time = get_attributes_from(html, ".footer .block .highlight", None, 12);
	let page_time = page_time.first().and_then(|s| {
		let parts: Vec<_> = s.split(' ').collect();
		if parts.len() == 2 && parts[1] == "seconds" {
			Some(parts[0])
		} else {
			None
		}
	});
	page_time.unwrap().to_string()
}

fn get_story_tags(html: &Html) -> Vec<StoryTag> {
	let mut tags = Vec::new();
	let selector = Selector::parse(".story-tags li a").unwrap();
	for child in html.select(&selector) {
		let tag = StoryTag {
			id: child.attr("tag-id").unwrap().parse::<u32>().unwrap(),
			title: child.attr("title").unwrap().into(),
			group: child.attr("class").unwrap().into(),
			href: child.attr("href").unwrap().into(),
			text: child.text().collect(),
		};
		tags.push(tag);
	}
	tags
}

fn parse_month(month: &str) -> u128 {
	match month {
		"Jan" => 1,
		"Feb" => 2,
		"Mar" => 3,
		"Apr" => 4,
		"May" => 5,
		"Jun" => 6,
		"Jul" => 7,
		"Aug" => 8,
		"Sep" => 9,
		"Oct" => 10,
		"Nov" => 11,
		"Dec" => 12,
		_ => unreachable!(),
	}
}

fn setup_database() -> Result<Connection, Box<dyn Error>> {
	let mut db = Connection::open("./fimfic-stats.db")?;
	let tx = db.transaction()?;
	tx.execute(include_str!("../queries/create/story-index.sql"), [])?;
	tx.execute(include_str!("../queries/create/authors.sql"), [])?;
	tx.execute(include_str!("../queries/create/stories.sql"), [])?;
	tx.execute(include_str!("../queries/create/tags.sql"), [])?;
	tx.execute(include_str!("../queries/create/tag-links.sql"), [])?;
	tx.execute(include_str!("../queries/create/chapters.sql"), [])?;
	tx.execute(include_str!("../queries/create/stats.sql"), [])?;
	tx.execute(include_str!("../queries/create/referral-sites.sql"), [])?;
	tx.execute(include_str!("../queries/create/referrals.sql"), [])?;
	tx.execute(include_str!("../queries/create/also-liked.sql"), [])?;
	tx.execute(include_str!("../queries/create/similar.sql"), [])?;
	tx.commit()?;
	Ok(db)
}
