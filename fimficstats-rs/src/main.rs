use self::structs::Api;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, COOKIE};
use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{env, time};

pub mod structs;

#[derive(Debug)]
enum Status {
	Published,
	Unpublished,
	Deleted,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let request_interval = 1000;

	let api_domain = "https://www.fimfiction.net/api/v2/stories";
	let stats_domain = "https://www.fimfiction.net/story/stats";
	let story_domain = "https://www.fimfiction.net/story";

	let token = &env::args().collect::<Vec<_>>()[1];
	let (api_client, api_headers) = setup_api_client(token)?;
	let (site_client, site_headers) = setup_site_client()?;

	for id in 1..=1000 {
		let start_time = time::SystemTime::now()
			.duration_since(UNIX_EPOCH)?
			.as_millis();

		let api_url = format!("{api_domain}/{id}");
		let api_response = api_client
			.get(api_url)
			.headers(api_headers.clone())
			.send()
			.await?;

		let stats_url = format!("{stats_domain}/{id}");
		let stats_response = site_client
			.get(stats_url)
			.headers(site_headers.clone())
			.send()
			.await?;

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
			Status::Unpublished => continue,
			Status::Deleted => continue,
			_ => {}
		}

		let story_url = format!("{story_domain}/{id}");
		let story_response = site_client
			.get(story_url)
			.headers(site_headers.clone())
			.send()
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
		sleep(start_time, request_interval).await
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

async fn sleep(start_time: u128, interval: u128) {
	let current_time = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis();
	let elapsed_time = current_time - start_time;
	if elapsed_time > interval {
		return;
	};
	tokio::time::sleep(Duration::from_millis((interval - elapsed_time) as u64)).await;
}
