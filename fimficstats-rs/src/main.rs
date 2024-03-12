use self::structs::Api;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let request_interval = 1000;

	let api_domain = "https://www.fimfiction.net/api/v2/stories";
	let stats_domain = "https://www.fimfiction.net/story/stats";

	let token = &env::args().collect::<Vec<_>>()[1];
	let client = Client::new();

	let mut headers = HeaderMap::new();
	headers.insert(
		AUTHORIZATION,
		HeaderValue::from_str(&format!("Bearer {}", token))?,
	);
	headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

	for id in 551751..=553110 {
		let start_time = time::SystemTime::now()
			.duration_since(UNIX_EPOCH)?
			.as_millis();

		let api_url = format!("{api_domain}/{id}");
		let api_response = client.get(api_url).headers(headers.clone()).send().await?;

		let stats_url = format!("{stats_domain}/{id}");
		let stats_response = client.get(stats_url).send().await?;

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
			 _ => {},
		}

		let api = api_response.json::<Api>().await;
		println!("{:#?}", api);
		println!("{id}: {status:?}");
		sleep(start_time, request_interval).await
	}

	Ok(())
}

async fn sleep(start_time: u128, interval: u128) {
	let current_time = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis();
	let elapsed_time = current_time - start_time;
	println!("{elapsed_time}");
	if elapsed_time > interval {
		return;
	};
	tokio::time::sleep(Duration::from_millis((interval - elapsed_time) as u64)).await;
}
