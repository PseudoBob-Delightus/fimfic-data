use self::structs::Api;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, COOKIE};
use reqwest::{Client, Response};
use scraper::{Html, Selector};
use std::env;
use std::error::Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub mod structs;

#[derive(Debug)]
enum Status {
	Published,
	Unpublished,
	Deleted,
}

const INTERVAL_STEP: u128 = 4000;
const INTERVAL_MAX: u128 = 120000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	// Set request intervals to ensure API and HTTPS calls are rate limited.
	let request_interval_short = 500;
	let request_interval_meduim = 1000;
	let request_interval_long = 1500;

	// Set the max number of consecutive deleted stories before stopping the script.
	let max_endpoint = 512;
	let mut current_endpoint = 0;

	let api_domain = "https://www.fimfiction.net/api/v2/stories";
	let stats_domain = "https://www.fimfiction.net/story/stats";
	let story_domain = "https://www.fimfiction.net/story";

	// API Bearer token is required to scrape the data.
	let token = &env::args().collect::<Vec<_>>()[1];

	let (api_client, api_headers) = setup_api_client(token)?;
	let (site_client, site_headers) = setup_site_client()?;

	// Loop over IDs to scrape data.
	for id in 1..=1000 {
		// End the script of we reach the max consecutive deleted stories.
		if current_endpoint > max_endpoint {
			break;
		}

		let start_time = unix_time()?;

		let api_url = format!("{api_domain}/{id}");
		let api_response = handle_request(
			request_interval_meduim,
			api_client.clone(),
			api_headers.clone(),
			&api_url,
		)
		.await?;

		let stats_url = format!("{stats_domain}/{id}");
		let stats_response = handle_request(
			request_interval_meduim,
			site_client.clone(),
			site_headers.clone(),
			&stats_url,
		)
		.await?;

		// Checks to see if the story is deleted or unpublished.
		let status = match (
			api_response.status().is_success(),
			stats_response.status().is_success(),
		) {
			(true, true) => Status::Published,
			(false, true) => Status::Unpublished,
			(false, false) => Status::Deleted,
			(true, false) => unreachable!(),
		};

		match status {
			Status::Deleted => {
				sleep(start_time, request_interval_short).await?;
				current_endpoint += 1;
				continue;
			}
			Status::Unpublished => {
				sleep(start_time, request_interval_meduim).await?;
				current_endpoint = 0;
				continue;
			}
			Status::Published => current_endpoint = 0,
		}

		let story_url = format!("{story_domain}/{id}");
		let story_response = handle_request(
			request_interval_meduim,
			site_client.clone(),
			site_headers.clone(),
			&story_url,
		)
		.await?;

		let html = Html::parse_document(&story_response.text().await?);
		let selector = Selector::parse("a.source").unwrap();

		if let Some(element) = html.select(&selector).next() {
			if let Some(link) = element.value().attr("href") {
				println!("{link}");
			}
		}

		let child_selector = Selector::parse("[data-story-id]").unwrap();

		let also_liked_selector = Selector::parse("[data-tab='also-liked']").unwrap();
		for parent in html.select(&also_liked_selector) {
			for child in parent.select(&child_selector) {
				if let Some(story_id) = child.value().attr("data-story-id") {
					println!("Also liked: {story_id}");
				}
			}
		}

		let similar_selector = Selector::parse("[data-tab='similar']").unwrap();
		for parent in html.select(&similar_selector) {
			for child in parent.select(&child_selector) {
				if let Some(story_id) = child.value().attr("data-story-id") {
					println!("Similar: {story_id}");
				}
			}
		}

		let _api = api_response.json::<Api>().await;
		// println!("{:#?}", api);
		println!("{id}: {status:?}");
		sleep(start_time, request_interval_long).await?
	}

	Ok(())
}

fn setup_api_client(token: &String) -> Result<(Client, HeaderMap), Box<dyn Error>> {
	let client = Client::new();
	let mut headers = HeaderMap::new();
	headers.insert(
		AUTHORIZATION,
		HeaderValue::from_str(&format!("Bearer {}", token))?,
	);
	headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
	Ok((client, headers))
}

fn setup_site_client() -> Result<(Client, HeaderMap), Box<dyn Error>> {
	let client = Client::new();
	let mut headers = HeaderMap::new();
	headers.insert(COOKIE, HeaderValue::from_static("view_mature=true"));
	Ok((client, headers))
}

async fn handle_request(
	interval: u128, client: Client, headers: HeaderMap, url: &str,
) -> Result<Response, Box<dyn Error>> {
	let mut interval = interval;
	loop {
		let start_time = unix_time()?;
		let res = send_http_request(client.clone(), headers.clone(), url).await;
		if res.is_ok() {
			return res;
		}
		sleep(start_time, interval).await?;
		interval = if interval < INTERVAL_MAX {
			interval + INTERVAL_STEP
		} else {
			INTERVAL_MAX
		};
		println!("Failed to send request to: {url}, next interval is: {interval} milliseconds.");
	}
}

async fn send_http_request(
	client: Client, headers: HeaderMap, url: &str,
) -> Result<Response, Box<dyn Error>> {
	Ok(client.get(url).headers(headers).send().await?)
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
