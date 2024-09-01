use self::structs::Api;
use chrono::Utc;
use pony::time::sleep_until_interval;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, COOKIE};
use reqwest::{Client, Response};
use rusqlite::{params, Connection};
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	println!("Program started at: {}", Utc::now());
	let program_start = unix_time()?;

	let version = 1.0;

	let mut db = setup_database()?;

	// URL setup.
	let fimfic = "https://www.fimfiction.net/api/v2/stories?page%5Bsize%5D=100";
	let heat_domain = format!("{fimfic}&sort=-hotness");
	let new_domain = format!("{fimfic}&sort=-date_published");
	let updated_domain = format!("{fimfic}&sort=-date_updated");

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

	let interval = 60_000;

	loop {
		let start_time = unix_time()?;
		let end_time = start_time % interval;
		sleep(start_time, Duration::from_millis(end_time as u64)).await;

		let api_response = handle_request(api.clone(), &api_url).await?;
	}
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
