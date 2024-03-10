#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import { Database } from "bun:sqlite";
import * as cheerio from "cheerio";
import * as sql from "./sql-patterns.ts";
import { Tag, api_schema, stats_schema } from "./types-and-schema.ts";
import * as plib from "./lib.ts";
import fs from "fs";

const db = new Database("./fimfic-stats.db", { create: true });
db.prepare(sql.story_index_table).run();
db.prepare(sql.authors_table).run();
db.prepare(sql.stories_table).run();
db.prepare(sql.tags_table).run();
db.prepare(sql.tag_links_table).run();
db.prepare(sql.chapters_table).run();
db.prepare(sql.stats_table).run();
db.prepare(sql.referral_sites_table).run();
db.prepare(sql.referrals_table).run();

await mane();

async function mane() {
	const version = 1;

	// API Bearer token is required to scrape the data.
	const access_token = process.argv[2];
	const api_domain = "https://www.fimfiction.net/api/v2/stories";
	const stats_domain = "https://www.fimfiction.net/story/stats";

	// Set a request interval to ensure API and HTTPS calls are rate limited.
	const request_interval = 1000;

	// Loop over IDs to scrape data.
	for (let id = 551751; id < 552652; id++) {
		const start_time = Date.now();
		let status = "unknown";

		// Set API and HTML status to -1.
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
			return response.json();
		});

		// Get html of the stats page.
		const stats_html = await fetch(`${stats_domain}/${id}`).then((response) => {
			html_status = response.status;
			return response.text();
		});

		// Checks to see if the story is deleted or unpublished.
		if (api_status === 200 && html_status === 200) {
			status = "published";
		} else if (api_status === 404 && html_status === 404) {
			status = "deleted";
		} else if (api_status === 404 && html_status === 200) {
			status = "unpublished";
		}

		console.log(`${id}: ${status}`);
		const table = sql.insert_story_index(id, status, version, start_time);
		db.query(table).run();

		if (status != "published") {
			await sleep(start_time, Date.now(), request_interval);
			continue;
		}

		const api = api_schema.parse(api_json);
		//console.dir(api, { depth: null });

		db.query(
			sql.insert_author(
				Number(api.data.relationships.author.data.id),
				api.included[0].attributes.name,
				new Date(api.included[0].attributes.date_joined).getTime() / 1000,
				api.included[0].attributes.num_followers,
				api.included[0].attributes.num_blog_posts,
			),
		).run();

		db.query(
			sql.insert_story(
				Number(api.data.id),
				api.data.attributes.title,
				new Date(api.data.attributes.date_modified).getTime() / 1000,
				new Date(api.data.attributes.date_updated).getTime() / 1000,
				new Date(api.data.attributes.date_published).getTime() / 1000,
				!!api.data.attributes.cover_image ? 1 : 0,
				api.data.attributes.color.hex,
				api.data.attributes.num_views,
				api.data.attributes.total_num_views,
				api.data.attributes.num_comments,
				api.data.attributes.rating,
				api.data.attributes.completion_status,
				api.data.attributes.content_rating,
				api.data.attributes.num_likes,
				api.data.attributes.num_dislikes,
				Number(api.data.relationships.author.data.id),
				!!api.data.relationships.prequel
					? Number(api.data.relationships.prequel.data.id)
					: "NULL",
			),
		).run();

		// Load the HTML with Cheerio.
		const document = cheerio.load(stats_html);

		// Get the tag IDs and names.
		let tags: Tag[] = [];
		document("ul.story-tags li").each((index, listItem) => {
			const tag = document(listItem).find("a");
			tags.push({
				id: Number(tag.attr("tag-id")),
				title: tag.attr("title")!,
				type: tag.attr("class")!,
				href: tag.attr("href")!,
				text: tag.text(),
			});
		});

		// Format the historical data into JSON.
		const data = document(".layout-two-columns[data-data]").attr("data-data")!;
		const stats = stats_schema.parse(JSON.parse(data));

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

		// Get the number of referrals from each site from the HTML.
		let referrals: Record<string, number> = {};

		document('h1:contains("Referrals")')
			.next("ul")
			.find("li")
			.each(function () {
				const [site, count] = document(this).text().split(": ");
				referrals[site] = Number(count);
			});

		// Log variables to console for testing.
		//console.log(tags);
		//console.log(referrals);
		//console.log(rating, word_ranking, bookshelves, tracking);
		//console.log(id, api_schema.parse(api_json));
		//console.dir(stats_schema.parse(JSON.parse(data!)), { depth: null });

		await sleep(start_time, Date.now(), request_interval);
	}
}

function sleep(
	start_time: number,
	current_time: number,
	interval: number,
): Promise<void> {
	const elapsed_time = current_time - start_time;
	if (elapsed_time > interval) return Promise.resolve();
	const remaining_time = interval - elapsed_time;
	return new Promise((res) => setTimeout(res, remaining_time));
}
