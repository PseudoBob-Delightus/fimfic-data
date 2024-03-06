#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import * as cheerio from "cheerio";
import * as plib from "./lib.ts";
import fs from "fs";

await mane();

async function mane() {
	// API Bearer token is required to scrape the data.
	const access_token = process.argv[2];
	const api_domain = "https://www.fimfiction.net/api/v2/stories";
	const stats_domain = "https://www.fimfiction.net/story/stats";

	// Loop over IDs to scrape data.
	for (let id = 551751; id < 552652; id++) {
		const start_time = Date.now();

		// Set API and HTML status to 200.
		let api_status = -1;
		let html_status = -1;

		// Get data from the API.
		const api_json = await fetch(`${api_domain}/${id}`, {
			method: "GET",
			headers: {
				Authorization: `Bearer ${access_token}`,
				"Content-Type": "application/json",
			},
		}).then((response) => {
			api_status = response.status;
			if (!response.ok) {
				console.error(`HTTP error! Status: ${response.status}`);
			}
			return response.json();
		});

		// Check for rate limiting.
		if (api_status === 429) {
			sleep(start_time, Date.now(), 5000);
			id = id - 1;
			continue;
		}

		// Get html of the stats page.
		const stats_html = await fetch(`${stats_domain}/${id}`).then((response) => {
			html_status = response.status;
			if (!response.ok) {
				console.error(`HTTP error! Status: ${response.status}`);
			}
			return response.text();
		});

		// Checks to see if the story is deleted or unpublished.
		if (api_status === 404 && html_status === 404) {
			console.warn("deleted story");
			sleep(start_time, Date.now(), 1000);
			continue;
		} else if (api_status === 404 && html_status === 200) {
			console.warn("unpublished story");
			sleep(start_time, Date.now(), 1000);
			// TODO: Add ID as unpublished and continue without scraping.
			continue;
		}

		// Load the HTML with Cheerio.
		const document = cheerio.load(stats_html);

		// Format the historical data into JSON.
		const data = document(".layout-two-columns[data-data]").attr("data-data");

		// Get the ranking and word count rankings from the HTML.
		const rankings = document('h1:contains("Rankings")').next("ul").find("li");
		const rating = Number(document(rankings[0]).text().replace(/\D/g, ""));
		const word_ranking = Number(
			document(rankings[1]).text().replace(/\D/g, ""),
		);

		// Get the number of bookshelves and tracking from the HTML.
		const books = document('h1:contains("Bookshelves")').next("ul").find("li");
		const bookshelves = Number(document(books[0]).text().replace(/\D/g, ""));
		const tracking = Number(document(books[1]).text().replace(/\D/g, ""));

		// Log variables to console for testing.
		console.log(rating, word_ranking, bookshelves, tracking);
		console.log(id, api_json);
		console.dir(JSON.parse(data!), { depth: null });

		sleep(start_time, Date.now(), 1000);
	}
}

function sleep(start_time: number, current_time: number, milliseconds: number) {
	const elapsed_time = current_time - start_time;
	console.log(elapsed_time);
	if (elapsed_time > milliseconds) return;
	const end_time = milliseconds - elapsed_time + current_time;
	while (Date.now() < end_time) {}
}
