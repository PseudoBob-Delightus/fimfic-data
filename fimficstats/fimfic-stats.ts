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
		// Set API and HTML status to 200.
		let api_status = 200;
		let html_status = 200;

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
			sleep(5000);
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
			continue;
		} else if (api_status === 404 && html_status === 200) {
			console.warn("unpublished story");
			// TODO: Add ID as unpublished and continue without scraping.
			continue;
		}

		// Load the HTML with Cheerio.
		const document = cheerio.load(stats_html);

		// Format the historical data into JSON.
		const data = document(".layout-two-columns[data-data]").attr("data-data");

		// Log variables to console for testing.
		console.log(id, api_json);
		console.dir(JSON.parse(data!), { depth: null });

		// Sleep for 1 second.
		sleep(1000);
	}
}

function sleep(milliseconds: number) {
	const date = Date.now();
	let current_date = null;
	do {
		current_date = Date.now();
	} while (current_date - date < milliseconds);
}
