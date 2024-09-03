use self::structs::Api;
use chrono::Utc;
use pony::time::{format_milliseconds, sleep_until_interval};
use pony::traits::compare;
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

const VERSION: f64 = 1.0;
const TYPES: &[(i32, &str)] = &[(0, "new"), (1, "updated"), (2, "heat"), (3, "featured")];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	println!("Program started at: {}", Utc::now());
	let program_start = unix_time()?;

	let mut db = setup_database()?;

	// URL setup.
	let fimfic = "https://www.fimfiction.net/api/v2/stories?page%5Bsize%5D=100";
	let heat_domain = format!("{fimfic}&sort=-hotness");
	let new_domain = format!("{fimfic}&sort=-date_published");
	let updated_domain = format!("{fimfic}&sort=-date_updated");
	let featured_domain = format!("{fimfic}&filter%5Bbookshelf%5D=1");

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

	let interval = 1_000;

	let mut iteration = db
		.query_row(
			include_str!("../queries/select/request-index.sql"),
			[],
			|row| row.get(0),
		)
		.unwrap_or(0)
		+ 1;

	loop {
		let start_time = unix_time()?;
		let end_time = start_time % interval;
		sleep(start_time, Duration::from_millis(end_time as u64)).await?;

		let one = unix_time()?;
		let new_response = handle_request(api.clone(), &new_domain).await?;
		let new = new_response.json::<Api>().await?;
		let two = unix_time()?;
		let updated_response = handle_request(api.clone(), &updated_domain).await?;
		let updated = updated_response.json::<Api>().await?;
		let three = unix_time()?;
		let heat_response = handle_request(api.clone(), &heat_domain).await?;
		let heat = heat_response.json::<Api>().await?;
		let four = unix_time()?;

		insert_request(
			&db,
			&new,
			iteration,
			100,
			one.try_into()?,
			two.try_into()?,
			0,
		)?;

		insert_request(
			&db,
			&updated,
			iteration,
			100,
			two.try_into()?,
			three.try_into()?,
			1,
		)?;

		insert_request(
			&db,
			&heat,
			iteration,
			100,
			three.try_into()?,
			four.try_into()?,
			2,
		)?;

		let mut stories = vec![];
		stories.extend(heat.data);
		stories.extend(new.data);
		stories.extend(updated.data);
		println!("Before: {}", stories.len());
		stories.sort_by(|a, b| compare(&a.id, &b.id));
		stories.dedup_by(|a, b| a.id == b.id);
		println!("After: {}", stories.len());

		let story_ids = stories.iter().map(|s| s.id.clone()).collect::<Vec<_>>();

		let featured = story_ids.chunks(100).map(|c| {
			let ids = c.join(",");
			let url = format!("{featured_domain}&filter%5Bids%5D={ids}");
		});

		let end_time = unix_time()?;
		let time = format_milliseconds(end_time - start_time, None)?;
		println!("Time: {time}");

		iteration += 1;
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

fn setup_database() -> Result<Connection, Box<dyn Error>> {
	let mut db = Connection::open("./fimfic-stats.db")?;
	let tx = db.transaction()?;
	tx.execute(include_str!("../queries/create/request-index.sql"), [])?;
	tx.execute(include_str!("../queries/create/request-type.sql"), [])?;
	tx.execute(include_str!("../queries/create/story-index.sql"), [])?;
	tx.execute(include_str!("../queries/create/authors.sql"), [])?;
	tx.execute(include_str!("../queries/create/stories.sql"), [])?;
	tx.execute(include_str!("../queries/create/tags.sql"), [])?;
	tx.execute(include_str!("../queries/create/tag-links.sql"), [])?;
	for r#type in TYPES {
		tx.execute(
			include_str!("../queries/insert/request-type.sql"),
			params![r#type.0, r#type.1],
		)?;
	}
	tx.commit()?;
	Ok(db)
}

fn insert_request(
	db: &Connection, request: &Api, iteration: usize, stories_requested: u32, start: u64, end: u64,
	type_id: u8,
) -> Result<(), Box<dyn Error>> {
	let mut tags = 0;
	let mut authors = 0;
	for attribute in &request.included {
		match attribute {
			structs::ApiIncluded::Tag(_) => tags += 1,
			structs::ApiIncluded::Author(_) => authors += 1,
		}
	}
	db.execute(
		include_str!("../queries/insert/request-index.sql"),
		params![
			type_id,
			iteration,
			VERSION,
			start,
			request.debug.duration,
			end - start,
			stories_requested,
			request.data.len(),
			tags,
			authors,
			request.meta.num_stories
		],
	)?;

	Ok(())
}
