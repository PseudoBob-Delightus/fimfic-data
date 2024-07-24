use self::structs::Api;
use pony::averages::SimpleMovingAverage;
use pony::time::format_milliseconds;
use pony::traits::OrderedVector;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, COOKIE};
use reqwest::{Client, Response};
use scraper::{Html, Selector};
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let program_start = unix_time()?;

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
		sleep(start_time, api.interval).await?;

		// Checks to see if the story is deleted or unpublished.
		if api_response.status().is_client_error() {
			times.insert((unix_time()? - start_time) as u32);
			current_endpoint += 1;
			continue;
		}

		current_endpoint = 0;

		let repsonse_time = unix_time()?;
		let stats_url = format!("{stats_domain}/{id}");
		let stats_response = handle_request(site.clone(), &stats_url).await?;
		sleep(repsonse_time, api.interval).await?;

		let repsonse_time = unix_time()?;
		let story_url = format!("{story_domain}/{id}");
		let story_response = handle_request(site.clone(), &story_url).await?;
		sleep(repsonse_time, api.interval).await?;

		let response = StoryResponse {
			api: api_response.json::<Api>().await?,
			stats: stats_response.text().await?,
			story: story_response.text().await?,
		};

		println!("Published story: {id}");
		parse_response(response).await;

		let end_time = unix_time()?;
		times.insert((end_time - start_time) as u32);
	}

	let program_end = unix_time()?;
	let time = format_milliseconds(program_end - program_start, None)?;
	println!("Total runtime: {time}");
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

async fn parse_response(response: StoryResponse) {
	parse_story_page(response.story);
}

fn parse_story_page(html: String) {
	let html = Html::parse_document(&html);
	let cover_source = get_attribute_if(&html, "a.source", Some("href"));
	println!("Cover source: {cover_source:?}");

	let also_liked = get_attributes_from(
		&html,
		"[data-tab='also-liked'] [data-story-id]",
		Some("data-story-id"),
		8,
	);
	println!("Also liked: {also_liked:?}");

	let similar = get_attributes_from(
		&html,
		"[data-tab='similar'] [data-story-id]",
		Some("data-story-id"),
		8,
	);
	println!("Similar: {similar:?}");

	let banned = get_attribute_if(&html, ".user-page-header .info-container a", Some("style"))
		.map_or(false, |style| style == "text-decoration:line-through");
	println!("Banned: {banned}");

	let offline_since = get_attribute_if(&html, ".mini-info-box [data-time]", Some("data-time"))
		.and_then(|time| time.parse::<u32>().ok());
	println!("Last online: {offline_since:?}");

	let groups = get_attribute_if(&html, ".header-groups .count", None)
		.map_or(0, |groups| groups.replace(',', "").parse::<u32>().unwrap());
	println!("Groups: {groups}");

	let stories = get_attribute_if(&html, ".tabs .tab-stories .number", None)
		.map_or(0, |stories| {
			stories.replace(',', "").parse::<u32>().unwrap()
		});
	println!("Stories: {stories}");

	let blogs = get_attribute_if(&html, ".tabs .tab-blog .number", None)
		.map_or(0, |blogs| blogs.replace(',', "").parse::<u32>().unwrap());
	println!("Blogs: {blogs}");

	let followers = get_attribute_if(&html, ".tabs .tab-followers .number", None)
		.map_or(0, |followers| {
			followers.replace(',', "").parse::<u32>().unwrap()
		});
	println!("Followers: {followers}");

	let following = get_attribute_if(&html, ".tabs .tab-following .number", None)
		.map_or(0, |following| {
			following.replace(',', "").parse::<u32>().unwrap()
		});
	println!("Following: {following}");
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
