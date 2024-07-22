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

pub mod structs;

#[derive(Debug, Clone)]
struct FimficRequest {
	client: Client,
	headers: HeaderMap,
	interval: u128,
	interval_step: u128,
	interval_max: u128,
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
		interval: 500,
		interval_step: 500,
		interval_max: 120_000,
	};
	let site = FimficRequest {
		client: Client::new(),
		headers: setup_site_headers()?,
		interval: 500,
		interval_step: 500,
		interval_max: 120_000,
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

		// Checks to see if the story is deleted or unpublished.
		if api_response.status().is_client_error() {
			sleep(start_time, api.interval).await?;
			times.insert((unix_time()? - start_time) as u32);
			current_endpoint += 1;
			continue;
		}

		sleep(start_time, api.interval).await?;
		let stats_url = format!("{stats_domain}/{id}");
		let _stats_response = handle_request(site.clone(), &stats_url).await?;

		current_endpoint = 0;

		sleep(start_time, api.interval).await?;
		let story_url = format!("{story_domain}/{id}");
		let story_response = handle_request(site.clone(), &story_url).await?;

		let _response_time = unix_time()?;

		let _api = api_response.json::<Api>().await?;

		let _response_time = unix_time()?;

		let html = Html::parse_document(&story_response.text().await?);
		let selector = Selector::parse("a.source").unwrap();

		if let Some(element) = html.select(&selector).next() {
			if let Some(_link) = element.value().attr("href") {
				//println!("{link}");
			}
		}

		let child_selector = Selector::parse("[data-story-id]").unwrap();

		let also_liked_selector = Selector::parse("[data-tab='also-liked']").unwrap();
		for parent in html.select(&also_liked_selector) {
			for child in parent.select(&child_selector) {
				if let Some(_story_id) = child.value().attr("data-story-id") {
					//println!("Also liked: {story_id}");
				}
			}
		}

		let similar_selector = Selector::parse("[data-tab='similar']").unwrap();
		for parent in html.select(&similar_selector) {
			for child in parent.select(&child_selector) {
				if let Some(_story_id) = child.value().attr("data-story-id") {
					//println!("Similar: {story_id}");
				}
			}
		}

		let _sleep_time = unix_time()?;
		sleep(start_time, api.interval).await?;
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
		let res = request
			.client
			.get(url)
			.headers(request.headers.clone())
			.send()
			.await;
		if res.is_ok() {
			return Ok(res?);
		}
		sleep(start_time, interval).await?;
		interval = if interval < request.interval_max {
			interval + request.interval_step
		} else {
			request.interval_max
		};
		println!("Failed to send request to: {url}, next interval is: {interval} milliseconds.");
	}
}

async fn sleep(start_time: u128, interval: u128) -> Result<(), Box<dyn Error>> {
	let current_time = unix_time()?;
	let elapsed_time = current_time - start_time;
	if elapsed_time > interval {
		return Ok(());
	};
	tokio::time::sleep(Duration::from_millis((interval - elapsed_time) as u64)).await;
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
