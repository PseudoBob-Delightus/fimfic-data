use self::structs::{Api, Stats};
use chrono::{TimeZone, Utc};
use pony::averages::SimpleMovingAverage;
use pony::time::format_milliseconds;
use pony::traits::OrderedVector;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, COOKIE};
use reqwest::{Client, Response};
use rusqlite::{Connection, Params};
use scraper::{ElementRef, Html, Selector};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::timeout;

pub mod structs;

#[derive(Debug, Clone)]
struct FimficRequest {
	client: Client,
	headers: HeaderMap,
	interval: Duration,
	interval_step: Duration,
	interval_max: Duration,
	timeout: Duration,
}

#[derive(Debug, Clone)]
struct StoryResponse {
	api: Api,
	stats: String,
	story: String,
}

#[derive(Debug, Clone)]
struct StoryData {
	api: Api,
	story: StoryPage,
	stats: StatsPage,
}

#[derive(Debug, Clone)]
struct StoryPage {
	banned: bool,
	offline_since: u128,
	following: u32,
	cover_source: String,
	also_liked: Vec<u32>,
	similar: Vec<u32>,
	groups: u32,
	page_time: u32,
}

#[derive(Debug, Clone)]
struct StatsPage {
	tags: Vec<StoryTag>,
	stats: Stats,
	ranking: u32,
	word_ranking: u32,
	bookshelves: u32,
	tracking: u32,
	referrals: HashMap<String, u32>,
	page_time: u32,
}

#[derive(Debug, Clone)]
struct StoryTag {
	id: u32,
	title: String,
	group: String,
	href: String,
	text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	println!("Program started at: {}", Utc::now());
	let program_start = unix_time()?;

	let db = setup_database()?;

	// Set the max number of consecutive deleted stories before stopping the script.
	let max_endpoint = 512;
	let mut current_endpoint = 0;

	// URL setup.
	let fimfic = "https://www.fimfiction.net";
	let latest_domain = format!("{fimfic}/stories?view_mode=2&sort=latest");
	let api_domain = format!("{fimfic}/api/v2/stories");
	let stats_domain = format!("{fimfic}/story/stats");
	let story_domain = format!("{fimfic}/story");

	// API Bearer token is required to scrape the data.
	let token = &env::args().collect::<Vec<_>>()[1];

	// API and site request structs, client, headers, and time intervals.
	let api = FimficRequest {
		client: Client::new(),
		headers: setup_api_headers(token)?,
		interval: Duration::from_millis(500),
		interval_step: Duration::from_secs(2),
		interval_max: Duration::from_secs(120),
		timeout: Duration::from_secs(10),
	};
	let site = FimficRequest {
		client: Client::new(),
		headers: setup_site_headers()?,
		interval: Duration::from_millis(500),
		interval_step: Duration::from_secs(2),
		interval_max: Duration::from_secs(120),
		timeout: Duration::from_secs(10),
	};

	// Simple weighted average times, used for estimating runtime.
	let mut times = SimpleMovingAverage::<u32>::new(10_000);

	// Get the latest ID for the time estimate.
	let ending_id = get_end_id(site.clone(), &latest_domain).await?;
	println!("{ending_id:?}");

	let start = 1;
	let end = start + 1_000;

	// Loop over IDs to scrape data.
	for id in start..=end {
		let start_time = unix_time()?;

		// End the script of we reach the max consecutive deleted stories.
		if current_endpoint > max_endpoint {
			break;
		}

		if !times.data.is_empty() {
			let average = times.average().unwrap();
			println!(
				"{id} -- real time: {}",
				format_milliseconds((average * (end - id)) as u128, None)?
			);
			println!(
				"{id} -- test time: {}",
				format_milliseconds((average * (ending_id.unwrap() + 512 - id)) as u128, None)?
			);
		}

		let api_url = format!("{api_domain}/{id}");
		let api_response = handle_request(api.clone(), &api_url).await?;

		// Checks to see if the story is deleted or unpublished.
		if api_response.status().is_client_error() {
			times.insert((unix_time()? - start_time) as u32);
			current_endpoint += 1;
			continue;
		}

		current_endpoint = 0;

		let stats_url = format!("{stats_domain}/{id}");
		let stats_response = handle_request(site.clone(), &stats_url).await?;

		let story_url = format!("{story_domain}/{id}");
		let story_response = handle_request(site.clone(), &story_url).await?;

		let response = StoryResponse {
			api: api_response.json::<Api>().await?,
			stats: stats_response.text().await?,
			story: story_response.text().await?,
		};

		let parse_start = unix_time()?;
		println!("Published story: {id}");
		parse_response(response).await;

		let end_time = unix_time()?;
		times.insert((end_time - start_time) as u32);

		println!(
			"Time to parse: {}",
			format_milliseconds(end_time - parse_start, None)?
		)
	}

	let program_end = unix_time()?;
	let time = format_milliseconds(program_end - program_start, None)?;
	println!("Total runtime: {time}");
	println!("Program ended at: {}", Utc::now());
	Ok(())
}

fn setup_api_headers(token: &str) -> Result<HeaderMap, Box<dyn Error>> {
	let mut headers = HeaderMap::new();
	headers.insert(
		AUTHORIZATION,
		HeaderValue::from_str(&format!("Bearer {}", token))?,
	);
	headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
	Ok(headers)
}

fn setup_site_headers() -> Result<HeaderMap, Box<dyn Error>> {
	let mut headers = HeaderMap::new();
	headers.insert(COOKIE, HeaderValue::from_static("view_mature=true"));
	Ok(headers)
}

async fn handle_request(request: FimficRequest, url: &str) -> Result<Response, Box<dyn Error>> {
	let mut interval = request.interval;
	loop {
		let start_time = unix_time()?;
		let res = timeout(
			request.timeout,
			request
				.client
				.get(url)
				.headers(request.headers.clone())
				.send(),
		)
		.await;
		match res {
			Ok(Ok(response)) => {
				sleep(start_time, request.interval).await?;
				return Ok(response);
			}
			Ok(Err(e)) => {
				println!("Request failed: {e}");
			}
			Err(e) => {
				println!("Request timed out: {e}");
			}
		}
		sleep(start_time, interval).await?;
		interval = if interval < request.interval_max {
			interval + request.interval_step
		} else {
			request.interval_max
		};
	}
}

async fn sleep(start_time: u128, interval: Duration) -> Result<(), Box<dyn Error>> {
	let current_time = unix_time()?;
	let elapsed_time = Duration::from_millis((current_time - start_time).try_into()?);
	if elapsed_time > interval {
		return Ok(());
	};
	tokio::time::sleep(interval - elapsed_time).await;
	Ok(())
}

fn unix_time() -> Result<u128, Box<dyn Error>> {
	Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())
}

async fn get_end_id(request: FimficRequest, url: &str) -> Result<Option<u32>, Box<dyn Error>> {
	let response = handle_request(request, url).await?;
	let html = Html::parse_document(&response.text().await?);
	let selector = Selector::parse("[data-story-id]")?;
	let mut ids = Vec::with_capacity(60);
	for element in html.select(&selector) {
		if let Some(story_id) = element.value().attr("data-story-id") {
			ids.push(story_id.parse::<u32>()?)
		}
	}
	Ok(ids.sort_vec().last().cloned())
}

async fn parse_response(response: StoryResponse) -> StoryData {
	StoryData {
		api: response.api,
		story: parse_story_page(response.story),
		stats: parse_stats_page(response.stats),
	}
}

fn parse_story_page(html: String) -> StoryPage {
	let html = Html::parse_document(&html);

	let banned = get_attribute_if(&html, ".user-page-header .info-container a", Some("style"))
		.map_or(false, |style| style == "text-decoration:line-through");

	let offline_since = get_attribute_if(&html, ".mini-info-box [data-time]", Some("title"))
		.map_or(unix_time().unwrap() / 1000, |time| parse_time(&time));

	let following = get_attribute_if(&html, ".tabs .tab-following .number", None)
		.map_or(0, |following| {
			following.replace(',', "").parse::<u32>().unwrap()
		});

	let cover_source =
		get_attribute_if(&html, "a.source", Some("href")).unwrap_or("NULL".to_string());

	let also_liked = get_attributes_from(
		&html,
		"[data-tab='also-liked'] [data-story-id]",
		Some("data-story-id"),
		8,
	)
	.iter()
	.map(|id| id.parse::<u32>().unwrap())
	.collect::<Vec<_>>();

	let similar = get_attributes_from(
		&html,
		"[data-tab='similar'] [data-story-id]",
		Some("data-story-id"),
		8,
	)
	.iter()
	.map(|id| id.parse::<u32>().unwrap())
	.collect::<Vec<_>>();

	let groups = get_attribute_if(&html, ".header-groups .count", None)
		.map_or(0, |groups| groups.replace(',', "").parse::<u32>().unwrap());
	println!("Groups: {groups}");

	let page_time = get_page_time(&html);

	StoryPage {
		banned,
		offline_since,
		following,
		cover_source,
		also_liked,
		similar,
		groups,
		page_time,
	}
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

fn parse_time(time: &str) -> u128 {
	let parts = time.split_whitespace().collect::<Vec<_>>();
	let year = parts[4].parse::<i32>().unwrap();
	let month = match parts[3] {
		"January" => 1,
		"February" => 2,
		"March" => 3,
		"April" => 4,
		"May" => 5,
		"June" => 6,
		"July" => 7,
		"August" => 8,
		"September" => 9,
		"October" => 10,
		"November" => 11,
		"December" => 12,
		_ => unreachable!(),
	};
	let day = parts[1]
		.chars()
		.filter(|c| c.is_ascii_digit())
		.collect::<String>()
		.parse::<u32>()
		.unwrap();
	let hour = parts[5]
		.split(':')
		.next()
		.unwrap()
		.trim_start_matches('@')
		.parse::<u32>()
		.unwrap();
	let hour = match parts[5] {
		part if part.ends_with("am") && hour == 12 => 0,
		part if part.ends_with("am") => hour,
		part if part.ends_with("pm") && hour != 12 => hour + 12,
		part if part.ends_with("pm") => hour,
		_ => unreachable!(),
	};
	let minute = parts[5]
		.split(':')
		.last()
		.unwrap()
		.chars()
		.filter(|c| c.is_ascii_digit())
		.collect::<String>()
		.parse::<u32>()
		.unwrap();

	Utc::with_ymd_and_hms(&Utc, year, month, day, hour, minute, 0)
		.unwrap()
		.timestamp()
		.try_into()
		.unwrap()
}

fn parse_stats_page(html: String) -> StatsPage {
	let html = Html::parse_document(&html);

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
		if let Some(parent_node) = child.parent() {
			if let Some(parent_element) = ElementRef::wrap(parent_node) {
				if let Some(attribute) = attribute {
					if let Some(text) = parent_element.value().attr(attribute) {
						attributes.push(text.into())
					}
				} else {
					attributes.push(parent_element.text().collect())
				}
			}
		}
	}
	attributes
}

fn get_right_stat(text: &str) -> u32 {
	text.split(':')
		.last()
		.unwrap()
		.chars()
		.filter(|c| c.is_ascii_digit())
		.collect::<String>()
		.parse::<u32>()
		.unwrap()
}

fn get_page_time(html: &Html) -> u32 {
	let page_time = get_attributes_from(html, ".footer .block .highlight", None, 12);
	let page_time = page_time.first().and_then(|s| {
		let parts: Vec<_> = s.split(' ').collect();
		if parts.len() == 2 && parts[1] == "seconds" {
			parts[0].parse::<f32>().ok()
		} else {
			None
		}
	});
	(page_time.unwrap() * 1000.0) as u32
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
