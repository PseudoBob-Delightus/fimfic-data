use chrono::{DateTime, NaiveDate, Utc};
use chrono::{Datelike, Weekday};
use pony::averages::SimpleMovingAverage;
use pony::number_format::format_number_f64;
use pony::number_format::format_number_u128;
use pony::time::format_milliseconds;
use rusqlite::Connection;
use rusqlite::OptionalExtension;
use rusqlite::params;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsPage {
	pub story_data: StoryData,
	pub author_data: AuthorData,
	pub sidebar_stats: SidebarStats,
	pub page_data: PageData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryData {
	pub id: u32,
	pub title: String,
	pub tags: Vec<StoryTag>,
	pub short_desc: String,
	pub published: Option<String>,
	pub stats: Stats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryTag {
	pub id: u32,
	pub title: String,
	pub group: String,
	pub href: String,
	pub text: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorData {
	pub id: u32,
	pub name: String,
	pub image_stub: String,
	pub bio: Option<String>,
	pub stories: u32,
	pub blogs: u32,
	pub followers: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidebarStats {
	pub views: u32,
	pub comments: u32,
	pub ranking: u32,
	pub word_ranking: u32,
	pub bookshelves: u32,
	pub tracking: u32,
	pub referrals: HashMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageData {
	pub timestamp: u32,
	pub page_time: f64,
	pub users_online: u32,
	pub hits_today: u32,
	pub hits_yesterday: u32,
}

#[derive(Debug, Clone, Default)]
pub struct DataStats {
	pub views: u32,
	pub likes: u32,
	pub dislikes: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	println!("Program started at: {}", Utc::now());
	let program_start = unix_time()?;

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

	let binding = story_map.clone();
	let first_date = binding.values().min().unwrap();
	let first_date = DateTime::from_timestamp_secs(*first_date as i64)
		.unwrap()
		.date_naive();

	let mut db = setup_database()?;

	let args: Vec<_> = env::args().collect();
	if let Some(arg) = args.get(1)
		&& (arg == "-s" || arg == "-stats")
	{
		story_data(&db, &first_date)?;
		stats_data(&db, &first_date)?;
		let program_end = unix_time()?;
		let time = format_milliseconds(program_end - program_start, None)?;
		println!("Total runtime: {time}");
		println!("Program ended at: {}", Utc::now());
		exit(0);
	}

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
		let stats = parse_stats_page(html, *id, *timestamp)?;

		let tx = db.transaction()?;

		tx.execute(
			include_str!("../queries/insert/stat-page.sql"),
			params![
				// story data
				stats.story_data.id,
				stats.story_data.title,
				stats.story_data.short_desc,
				stats.story_data.published,
				parse_chapter_date(stats.story_data.stats.stats.first_chapter_date)?,
				parse_chapter_date(stats.story_data.stats.stats.last_chapter_date)?,
				// author data
				stats.author_data.id,
				stats.author_data.name,
				stats.author_data.image_stub,
				stats.author_data.bio,
				stats.author_data.stories,
				stats.author_data.blogs,
				stats.author_data.followers,
				// sidebar data
				stats.sidebar_stats.views,
				stats.sidebar_stats.comments,
				stats.sidebar_stats.ranking,
				stats.sidebar_stats.word_ranking,
				stats.sidebar_stats.bookshelves,
				stats.sidebar_stats.tracking,
				// page data
				stats.page_data.timestamp,
				stats.page_data.page_time,
				stats.page_data.users_online,
				stats.page_data.hits_today,
				stats.page_data.hits_yesterday,
			],
		)?;

		for tag in stats.story_data.tags {
			tx.execute(
				include_str!("../queries/insert/tag.sql"),
				(tag.id, tag.title, tag.group, tag.text, tag.href),
			)?;
			tx.execute(include_str!("../queries/insert/tag-link.sql"), (id, tag.id))?;
		}

		for chapter in stats.story_data.stats.chapters {
			tx.execute(
				include_str!("../queries/insert/chapter.sql"),
				(
					id,
					chapter.chapter_num,
					chapter.title,
					chapter.views,
					chapter.words,
					chapter.date,
				),
			)?;
		}

		for stat in stats.story_data.stats.stats.data {
			tx.execute(
				include_str!("../queries/insert/stats.sql"),
				(id, stat.views, stat.likes, stat.dislikes, stat.date),
			)?;
		}

		for (ref site, count) in stats.sidebar_stats.referrals {
			let site_id: Option<u32> = tx
				.query_one(
					include_str!("../queries/select/referral-site.sql"),
					(site,),
					|row| row.get(0),
				)
				.optional()?;
			let site_id = if let Some(site_id) = site_id {
				site_id
			} else {
				tx.query_one(
					include_str!("../queries/insert/referral-site.sql"),
					(site,),
					|row| row.get(0),
				)?
			};
			tx.execute(
				include_str!("../queries/insert/referral.sql"),
				(id, site_id, count),
			)?;
		}

		tx.commit()?;

		if !times.data.is_empty() {
			let percent = (i as f64 / total as f64) * 100.0;
			let average = times.average().unwrap();
			println!(
				"Iteration: {i:6}, percentage: {percent:5.2}, ID: {id:6}, estimated time remaining: {}",
				format_milliseconds((average * (total - i) as u32) as u128, Some(2))?
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

fn parse_stats_page(
	html: String, story_id: u32, timestamp: u32,
) -> Result<StatsPage, Box<dyn Error>> {
	let html = Html::parse_document(&html);

	let story_title = get_attributes_from(&html, ".title a", None);
	let story_title = story_title[0].clone();

	let short_desc = get_attributes_from(&html, ".story-page-header .desktop p", None);
	let short_desc = short_desc.first().unwrap().to_owned();

	let published =
		get_attributes_from_parent(&html, ".story-page-header .mini-info-box li > b", None, 1);
	let published = if !published.is_empty() {
		let parts: Vec<_> = published[0].split_whitespace().collect();
		let day = parts[1].trim_end_matches(|c: char| !c.is_ascii_digit());
		let month = parse_month(parts[2]);
		let year = parts[3];
		Some(format!("{year}-{month}-{day}"))
	} else {
		None
	};

	let stats = get_attribute_if(&html, ".layout-two-columns.story-stats", Some("data-data"));
	let stats = serde_json::from_str::<Stats>(&stats.unwrap()).unwrap();

	let story_data = StoryData {
		id: story_id,
		title: story_title,
		tags: get_story_tags(&html),
		short_desc,
		published,
		stats,
	};

	let author_id = get_attributes_from(&html, ".story-page-header .author a", Some("href"));
	let author_id = author_id[0].split('/').nth(2).unwrap().parse().unwrap();
	let author_name = get_attributes_from(&html, ".story-page-header .author a", None);
	let author_name = author_name[0].clone();
	let author_image_stub = get_attributes_from(&html, ".user-card img", Some("data-src"));
	let author_image_stub = author_image_stub[0]
		.split('/')
		.next_back()
		.unwrap()
		.to_string();
	let author_bio = get_attributes_from(&html, ".user-card .info p", None);
	let author_bio = if !author_bio.is_empty() {
		Some(author_bio[0].clone())
	} else {
		None
	};

	let author_stats = get_attributes_from(&html, ".user-links .number", None);

	let author_data = AuthorData {
		id: author_id,
		name: author_name,
		image_stub: author_image_stub,
		bio: author_bio,
		stories: author_stats[0].clone().replace(',', "").parse()?,
		blogs: author_stats[1].clone().replace(',', "").parse()?,
		followers: author_stats[2].clone().replace(',', "").parse()?,
	};

	let sidebar_stats =
		get_attributes_from_parent(&html, ".content_box .article ul li > b", None, 6);
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

	let sidebar_stats = SidebarStats {
		views: sidebar_stats.first().map(|s| get_right_stat(s)).unwrap(),
		comments: sidebar_stats.get(1).map(|s| get_right_stat(s)).unwrap(),
		ranking: sidebar_stats.get(2).map(|s| get_right_stat(s)).unwrap(),
		word_ranking: sidebar_stats.get(3).map(|s| get_right_stat(s)).unwrap(),
		bookshelves: sidebar_stats.get(4).map(|s| get_right_stat(s)).unwrap(),
		tracking: sidebar_stats.get(5).map(|s| get_right_stat(s)).unwrap(),
		referrals,
	};

	let page_stats = get_attributes_from(&html, ".footer .block .highlight", None);
	let page_data = PageData {
		timestamp,
		page_time: page_stats[0].split(' ').next().unwrap().parse()?,
		users_online: page_stats[2].clone().replace(',', "").parse()?,
		hits_today: page_stats[3].clone().replace(',', "").parse()?,
		hits_yesterday: page_stats[4].clone().replace(',', "").parse()?,
	};

	Ok(StatsPage {
		story_data,
		author_data,
		sidebar_stats,
		page_data,
	})
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

fn get_attributes_from(html: &Html, from: &str, attribute: Option<&str>) -> Vec<String> {
	let mut attributes = Vec::new();
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

fn parse_chapter_date(date: ChapterDate) -> Result<Option<u32>, Box<dyn Error>> {
	if let ChapterDate::Text(date) = date {
		Ok(Some(date.parse::<u32>()?))
	} else {
		Ok(None)
	}
}

fn setup_database() -> Result<Connection, Box<dyn Error>> {
	let mut db = Connection::open("/home/velvetremedy/fimfic/fimfic-stats.db")?;
	let tx = db.transaction()?;
	tx.execute(include_str!("../queries/create/stat-pages.sql"), [])?;
	tx.execute(include_str!("../queries/create/tags.sql"), [])?;
	tx.execute(include_str!("../queries/create/tag-links.sql"), [])?;
	tx.execute(include_str!("../queries/create/chapters.sql"), [])?;
	tx.execute(include_str!("../queries/create/stats.sql"), [])?;
	tx.execute(include_str!("../queries/create/referral-sites.sql"), [])?;
	tx.execute(include_str!("../queries/create/referrals.sql"), [])?;
	tx.commit()?;
	Ok(db)
}

pub fn update_data_stat(stats: &mut DataStats, data: &StatsData) {
	if let Some(count) = data.views {
		stats.views += count;
	}
	if let Some(count) = data.likes {
		stats.likes += count;
	}
	if let Some(count) = data.dislikes {
		stats.dislikes += count;
	}
}

fn story_data(db: &Connection, first_date: &NaiveDate) -> Result<(), Box<dyn Error>> {
	let mut stmt =
		db.prepare("SELECT story_id, date_published, first_chapter_date, views FROM stat_pages;")?;
	let stats_iter = stmt.query_map([], |row| {
		Ok((
			row.get::<_, u32>(0)?,
			row.get::<_, Option<String>>(1)?,
			row.get::<_, Option<i64>>(2)?,
			row.get::<_, u32>(3)?,
		))
	})?;

	let mut story_data = HashMap::new();

	for data in stats_iter {
		let (story_id, date, chapter_date, views) = data?;
		let date = match date {
			Some(date) => &NaiveDate::parse_from_str(&date, "%Y-%m-%d")?,
			None => &DateTime::from_timestamp_secs(chapter_date.unwrap())
				.unwrap()
				.date_naive(),
		};
		if first_date <= date {
			continue;
		}
		let mut stmt = db.prepare("SELECT views FROM chapters WHERE story_id = :id;")?;
		let stats_iter = stmt.query_map(&[(":id", &story_id.to_string())], |row| {
			row.get::<_, u32>(0)
		})?;
		let mut total = 0;
		for chapter in stats_iter {
			total += chapter?;
		}
		let weekday = date.weekday();
		story_data
			.entry(weekday as u8)
			.and_modify(|data: &mut Vec<_>| data.push((views, total)))
			.or_insert_with(|| vec![(views, total)]);
	}

	for day in [6, 0, 1, 2, 3, 4, 5] {
		let data = story_data.get(&day).unwrap();
		let count = data.len();
		let mut views_total = 0;
		let mut total_views_total = 0;
		for (views, total_views) in data {
			views_total += views;
			total_views_total += total_views;
		}
		println!("=======================================");
		println!(
			"Day of week: {}, total stories: {}",
			Weekday::try_from(day)?,
			format_number_u128(count as u128)?
		);
		println!(
			"views - total: {}, average: {}",
			format_number_u128(views_total as u128)?,
			format_number_f64(views_total as f64 / count as f64, 4)?
		);
		println!(
			"total views - total: {}, average: {}",
			format_number_u128(total_views_total as u128)?,
			format_number_f64(total_views_total as f64 / count as f64, 4)?
		);
	}

	let mut stmt = db.prepare(
		"SELECT date_published, first_chapter_date, length(title), comments, bookshelves, tracking FROM stat_pages;",
	)?;
	let stats_iter = stmt.query_map([], |row| {
		Ok((
			row.get::<_, Option<String>>(0)?,
			row.get::<_, Option<i64>>(1)?,
			row.get::<_, u32>(2)?,
			row.get::<_, u32>(3)?,
			row.get::<_, u32>(4)?,
			row.get::<_, u32>(5)?,
		))
	})?;
	let mut story_data = HashMap::new();
	for stats in stats_iter {
		let (date, chapter_date, title_len, comments, bookshelves, tracking) = stats?;
		let date = match date {
			Some(date) => &NaiveDate::parse_from_str(&date, "%Y-%m-%d")?,
			None => &DateTime::from_timestamp_secs(chapter_date.unwrap())
				.unwrap()
				.date_naive(),
		};
		if first_date <= date {
			continue;
		}
		story_data
			.entry(date.year())
			.and_modify(|data: &mut Vec<_>| data.push((title_len, comments, bookshelves, tracking)))
			.or_insert_with(|| vec![(title_len, comments, bookshelves, tracking)]);
	}

	for year in 2011..2024 {
		let data = story_data.get(&year).unwrap();
		let story_count = data.len();
		let mut title_length = 0;
		let mut comments = 0;
		let mut bookshelves = 0;
		let mut tracking = 0;

		for story in data {
			title_length += story.0;
			comments += story.1;
			bookshelves += story.2;
			tracking += story.3;
		}

		println!("=======================================");
		println!(
			"Year: {year}, total stories: {}",
			format_number_u128(story_count as u128)?
		);
		println!(
			"title length - total: {}, average: {}",
			format_number_u128(title_length as u128)?,
			format_number_f64(title_length as f64 / story_count as f64, 4)?
		);
		println!(
			"comments - total: {}, average: {}",
			format_number_u128(comments as u128)?,
			format_number_f64(comments as f64 / story_count as f64, 4)?
		);
		println!(
			"bookshelves - total: {}, average: {}",
			format_number_u128(bookshelves as u128)?,
			format_number_f64(bookshelves as f64 / story_count as f64, 4)?
		);
		println!(
			"tracking - total: {}, average: {}",
			format_number_u128(tracking as u128)?,
			format_number_f64(tracking as f64 / story_count as f64, 4)?
		);
	}

	Ok(())
}

fn stats_data(db: &Connection, first_date: &NaiveDate) -> Result<(), Box<dyn Error>> {
	let mut stmt =
		db.prepare("SELECT story_id, date_published, first_chapter_date FROM stat_pages;")?;
	let stats_iter = stmt.query_map([], |row| {
		Ok((
			row.get::<_, u32>(0)?,
			row.get::<_, Option<String>>(1)?,
			row.get::<_, Option<i64>>(2)?,
		))
	})?;

	let mut story_years = HashMap::new();
	let mut story_tags = HashMap::new();

	let mut stmt = db.prepare("SELECT tag_id FROM tag_links WHERE story_id = :id;")?;
	for stat in stats_iter {
		let (story_id, date, chapter_date) = stat?;
		let date = match date {
			Some(date) => &NaiveDate::parse_from_str(&date, "%Y-%m-%d")?,
			None => &DateTime::from_timestamp_secs(chapter_date.unwrap())
				.unwrap()
				.date_naive(),
		};
		let tags = stmt
			.query_map(&[(":id", &story_id.to_string())], |row| {
				row.get::<_, u32>(0)
			})?
			.map(|stat| stat.unwrap())
			.collect::<Vec<_>>();
		story_tags.insert(story_id, tags);
		story_years.insert(story_id, date.year());
	}

	let mut stmt = db.prepare("SELECT story_id, views, likes, dislikes, date FROM stats;")?;
	let stats_iter = stmt.query_map([], |row| {
		Ok((
			row.get::<_, u32>(0)?,
			StatsData {
				views: row.get::<_, Option<u32>>(1)?,
				likes: row.get::<_, Option<u32>>(2)?,
				dislikes: row.get::<_, Option<u32>>(3)?,
				date: row.get::<_, String>(4)?,
			},
		))
	})?;

	let mut dates = HashSet::new();
	let mut total_stats = DataStats::default();
	let mut year_stats = HashMap::new();
	let mut weekday_stats = HashMap::new();
	let mut view_year_stats = HashMap::new();
	let mut story_year_stats = HashMap::new();
	let mut tag_year_stats = HashMap::new();

	for stat in stats_iter {
		let (story_id, stat) = stat?;
		let date = &NaiveDate::parse_from_str(&stat.date, "%Y-%m-%d")?;
		if first_date <= date {
			continue;
		}
		dates.insert(stat.date.clone());
		update_data_stat(&mut total_stats, &stat);
		let year = date.year();
		year_stats
			.entry(year)
			.and_modify(|data| update_data_stat(data, &stat))
			.or_insert_with(|| {
				let mut data = DataStats::default();
				update_data_stat(&mut data, &stat);
				data
			});
		let weekday = date.weekday();
		weekday_stats
			.entry(weekday as i8)
			.and_modify(|data| update_data_stat(data, &stat))
			.or_insert_with(|| {
				let mut data = DataStats::default();
				update_data_stat(&mut data, &stat);
				data
			});
		let publish_year = story_years.get(&story_id).unwrap();
		view_year_stats
			.entry(year)
			.and_modify(|data: &mut HashMap<_, _>| {
				data.entry(publish_year)
					.and_modify(|data| update_data_stat(data, &stat))
					.or_insert_with(|| {
						let mut data = DataStats::default();
						update_data_stat(&mut data, &stat);
						data
					});
			})
			.or_insert_with(|| {
				let mut data = DataStats::default();
				update_data_stat(&mut data, &stat);
				let mut year_map = HashMap::new();
				year_map.insert(publish_year, data);
				year_map
			});
		story_year_stats
			.entry(year)
			.and_modify(|data: &mut HashMap<_, _>| {
				data.entry(story_id)
					.and_modify(|data| update_data_stat(data, &stat))
					.or_insert_with(|| {
						let mut data = DataStats::default();
						update_data_stat(&mut data, &stat);
						data
					});
			})
			.or_insert_with(|| {
				let mut data = DataStats::default();
				update_data_stat(&mut data, &stat);
				let mut year_map = HashMap::new();
				year_map.insert(story_id, data);
				year_map
			});
		let tags = story_tags.get(&story_id).unwrap();
		for tag_id in tags {
			tag_year_stats
				.entry(year)
				.and_modify(|data: &mut HashMap<_, _>| {
					data.entry(tag_id)
						.and_modify(|data| update_data_stat(data, &stat))
						.or_insert_with(|| {
							let mut data = DataStats::default();
							update_data_stat(&mut data, &stat);
							data
						});
				})
				.or_insert_with(|| {
					let mut data = DataStats::default();
					update_data_stat(&mut data, &stat);
					let mut tag_map = HashMap::new();
					tag_map.insert(tag_id, data);
					tag_map
				});
		}
	}

	let mut dates: Vec<_> = dates.iter().cloned().collect();
	dates.sort();
	let first = dates.first().unwrap();
	let last = dates.last().unwrap();
	let first = NaiveDate::parse_from_str(first, "%Y-%m-%d")?;
	let last = NaiveDate::parse_from_str(last, "%Y-%m-%d")?;

	println!("=======================================");
	println!("Span: {first} - {last}");

	let days = dates.len();

	println!(
		"views - total: {}, average: {}",
		format_number_u128(total_stats.views as u128)?,
		format_number_f64(total_stats.views as f64 / days as f64, 4)?
	);
	println!(
		"likes - total: {}, average: {}",
		format_number_u128(total_stats.likes as u128)?,
		format_number_f64(total_stats.likes as f64 / days as f64, 4)?
	);
	println!(
		"dislikes - total: {}, average: {}",
		format_number_u128(total_stats.dislikes as u128)?,
		format_number_f64(total_stats.dislikes as f64 / days as f64, 4)?
	);

	for year in 2011..2024 {
		let data = year_stats.get(&year).unwrap();
		let publish_data = view_year_stats.get(&year).unwrap();
		let mut publish_data = publish_data.iter().collect::<Vec<_>>();
		publish_data.sort_by_key(|(year, _)| *year);
		let story_data = story_year_stats.get(&year).unwrap();
		let mut story_data = story_data.iter().collect::<Vec<_>>();
		story_data.sort_by_key(|(_, stat)| stat.views);
		story_data.reverse();
		let views_25 = story_data.iter().take(25);
		println!("=======================================");
		for (i, (story_id, stats)) in views_25.enumerate() {
			let publish_year = story_years.get(*story_id).unwrap();
			let mut stmt =
				db.prepare("SELECT title FROM stat_pages WHERE story_id = :story_id LIMIT 1;")?;
			let title = stmt.query_one(&[(":story_id", &story_id.to_string())], |row| {
				row.get::<_, String>(0)
			})?;
			println!("Top {} story by views: {title}", i + 1);
			println!(
				"\tviews: {}, likes: {}, dislikes: {}, published: {publish_year}",
				stats.views, stats.likes, stats.dislikes
			)
		}
		story_data.sort_by_key(|(_, stat)| stat.likes);
		story_data.reverse();
		let likes_25 = story_data.iter().take(25);
		println!("=======================================");
		for (i, (story_id, stats)) in likes_25.enumerate() {
			let publish_year = story_years.get(*story_id).unwrap();
			let mut stmt =
				db.prepare("SELECT title FROM stat_pages WHERE story_id = :story_id LIMIT 1;")?;
			let title = stmt.query_one(&[(":story_id", &story_id.to_string())], |row| {
				row.get::<_, String>(0)
			})?;
			println!("Top {} story by likes: {title}", i + 1);
			println!(
				"\tviews: {}, likes: {}, dislikes: {}, published: {publish_year}",
				stats.views, stats.likes, stats.dislikes
			)
		}
		story_data.sort_by_key(|(_, stat)| stat.dislikes);
		story_data.reverse();
		let dislikes_25 = story_data.iter().take(25);
		println!("=======================================");
		for (i, (story_id, stats)) in dislikes_25.enumerate() {
			let publish_year = story_years.get(*story_id).unwrap();
			let mut stmt =
				db.prepare("SELECT title FROM stat_pages WHERE story_id = :story_id LIMIT 1;")?;
			let title = stmt.query_one(&[(":story_id", &story_id.to_string())], |row| {
				row.get::<_, String>(0)
			})?;
			println!("Top {} story by dislikes: {title}", i + 1);
			println!(
				"\tviews: {}, likes: {}, dislikes: {}, published: {publish_year}",
				stats.views, stats.likes, stats.dislikes
			)
		}

		let tag_data = tag_year_stats.get(&year).unwrap();
		let mut tag_data = tag_data.iter().collect::<Vec<_>>();
		tag_data.sort_by_key(|(_, stat)| stat.views);
		tag_data.reverse();
		let views_25 = tag_data.iter().take(25);
		println!("=======================================");
		for (i, (tag_id, stats)) in views_25.enumerate() {
			let mut stmt = db.prepare("SELECT text FROM tags WHERE id = :id LIMIT 1;")?;
			let text = stmt.query_one(&[(":id", &tag_id.to_string())], |row| {
				row.get::<_, String>(0)
			})?;
			println!("Top {} tag by views: {text}", i + 1);
			println!(
				"\tviews: {}, likes: {}, dislikes: {}",
				stats.views, stats.likes, stats.dislikes
			)
		}
		tag_data.sort_by_key(|(_, stat)| stat.likes);
		tag_data.reverse();
		let likes_25 = tag_data.iter().take(25);
		println!("=======================================");
		for (i, (tag_id, stats)) in likes_25.enumerate() {
			let mut stmt = db.prepare("SELECT text FROM tags WHERE id = :id LIMIT 1;")?;
			let title = stmt.query_one(&[(":id", &tag_id.to_string())], |row| {
				row.get::<_, String>(0)
			})?;
			println!("Top {} tag by likes: {title}", i + 1);
			println!(
				"\tviews: {}, likes: {}, dislikes: {}",
				stats.views, stats.likes, stats.dislikes
			)
		}
		tag_data.sort_by_key(|(_, stat)| stat.dislikes);
		tag_data.reverse();
		let dislikes_25 = tag_data.iter().take(25);
		println!("=======================================");
		for (i, (tag_id, stats)) in dislikes_25.enumerate() {
			let mut stmt = db.prepare("SELECT text FROM tags WHERE id = :id LIMIT 1;")?;
			let title = stmt.query_one(&[(":id", &tag_id.to_string())], |row| {
				row.get::<_, String>(0)
			})?;
			println!("Top {} tag by dislikes: {title}", i + 1);
			println!(
				"\tviews: {}, likes: {}, dislikes: {}",
				stats.views, stats.likes, stats.dislikes
			)
		}

		let mut dates: Vec<_> = dates
			.iter()
			.filter(|&date| date.starts_with(&year.to_string()))
			.cloned()
			.collect();
		dates.sort();
		let first = dates.first().unwrap();
		let last = dates.last().unwrap();
		let first = NaiveDate::parse_from_str(first, "%Y-%m-%d")?;
		let last = NaiveDate::parse_from_str(last, "%Y-%m-%d")?;

		println!("=======================================");
		println!("Span: {first} - {last}");

		let days = (last - first).num_days();

		println!(
			"views - total: {}, average: {}",
			format_number_u128(data.views as u128)?,
			format_number_f64(data.views as f64 / days as f64, 4)?
		);
		println!(
			"views by year:\n\t{}",
			publish_data
				.iter()
				.map(|(year, stats)| format!(
					"{year}: {}%, total: {}",
					format_number_f64(stats.views as f64 / data.views as f64 * 100.0, 4).unwrap(),
					format_number_u128(stats.views as u128).unwrap(),
				))
				.collect::<Vec<_>>()
				.join("\n\t")
		);

		println!(
			"likes - total: {}, average: {}",
			format_number_u128(data.likes as u128)?,
			format_number_f64(data.likes as f64 / days as f64, 4)?
		);
		println!(
			"likes by year:\n\t{}",
			publish_data
				.iter()
				.map(|(year, stats)| format!(
					"{year}: {}%, total: {}",
					format_number_f64(stats.likes as f64 / data.likes as f64 * 100.0, 4).unwrap(),
					format_number_u128(stats.likes as u128).unwrap(),
				))
				.collect::<Vec<_>>()
				.join("\n\t")
		);

		println!(
			"dislikes - total: {}, average: {}",
			format_number_u128(data.dislikes as u128)?,
			format_number_f64(data.dislikes as f64 / days as f64, 4)?
		);
		println!(
			"dislikes by year:\n\t{}",
			publish_data
				.iter()
				.map(|(year, stats)| format!(
					"{year}: {}%, total: {}",
					format_number_f64(stats.dislikes as f64 / data.dislikes as f64 * 100.0, 4)
						.unwrap(),
					format_number_u128(stats.dislikes as u128).unwrap(),
				))
				.collect::<Vec<_>>()
				.join("\n\t")
		);
	}

	for day in [6, 0, 1, 2, 3, 4, 5] {
		let data = weekday_stats.get(&day).unwrap();
		let mut dates: Vec<_> = dates
			.iter()
			.filter(|&date| {
				NaiveDate::parse_from_str(date, "%Y-%m-%d")
					.unwrap()
					.weekday() as i8
					== day
			})
			.cloned()
			.collect();
		dates.sort();
		let first = dates.first().unwrap();
		let first = NaiveDate::parse_from_str(first, "%Y-%m-%d")?;
		let days = dates.len();

		println!("=======================================");
		println!("Day of week: {}, total days: {days}", first.weekday());

		println!(
			"views - total: {}, average: {}",
			format_number_u128(data.views as u128)?,
			format_number_f64(data.views as f64 / days as f64, 4)?
		);
		println!(
			"likes - total: {}, average: {}",
			format_number_u128(data.likes as u128)?,
			format_number_f64(data.likes as f64 / days as f64, 4)?
		);
		println!(
			"dislikes - total: {}, average: {}",
			format_number_u128(data.dislikes as u128)?,
			format_number_f64(data.dislikes as f64 / days as f64, 4)?
		);
	}
	Ok(())
}
