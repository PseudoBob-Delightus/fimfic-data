#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import { Database } from "bun:sqlite";
import * as cheerio from "cheerio";
import z from "zod";
import * as sql from "./sql-patterns.ts";
import * as plib from "./lib.ts";
import fs from "fs";
import { version } from "bun";

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

// Schema for the story API response.
const api_schema = z.object({
	data: z.object({
		id: z.string(),
		type: z.string(),
		attributes: z.object({
			title: z.string(),
			short_description: z.string(),
			description: z.string(),
			description_html: z.string(),
			date_modified: z.string(),
			date_updated: z.string(),
			date_published: z.string(),
			published: z.boolean(),
			cover_image: z
				.object({
					thumbnail: z.string(),
					medium: z.string(),
					large: z.string(),
					full: z.string(),
				})
				.optional(),
			color: z.object({
				hex: z.string(),
				rgb: z.array(z.number()).min(3).max(3),
			}),
			num_views: z.number(),
			total_num_views: z.number(),
			num_words: z.number(),
			num_chapters: z.number(),
			num_comments: z.number(),
			rating: z.number(),
			status: z.string(),
			submitted: z.boolean(),
			completion_status: z.string(),
			content_rating: z.string(),
			num_likes: z.number(),
			num_dislikes: z.number(),
		}),
		relationships: z.object({
			author: z.object({
				data: z.object({
					type: z.string(),
					id: z.string(),
				}),
			}),
			tags: z.object({
				data: z.array(
					z.object({
						type: z.string(),
						id: z.string(),
					}),
				),
			}),
			prequel: z
				.object({
					data: z.object({
						type: z.string(),
						id: z.string(),
					}),
				})
				.optional(),
		}),
		links: z.object({
			self: z.string(),
		}),
		meta: z.object({
			url: z.string(),
		}),
	}),
	included: z.array(
		z.object({
			id: z.string(),
			type: z.string(),
			attributes: z.object({
				name: z.string(),
				bio: z.string(),
				bio_html: z.string(),
				num_followers: z.number(),
				num_stories: z.number(),
				num_blog_posts: z.number(),
				avatar: z.object({
					32: z.string(),
					48: z.string(),
					64: z.string(),
					96: z.string(),
					128: z.string(),
					160: z.string(),
					192: z.string(),
					256: z.string(),
					320: z.string(),
					384: z.string(),
					512: z.string(),
				}),
				color: z.object({
					hex: z.string(),
					rgb: z.array(z.number()).min(3).max(3),
				}),
				date_joined: z.string(),
			}),
			links: z.object({
				self: z.string(),
			}),
			meta: z.object({
				url: z.string(),
			}),
		}),
	),
	uri: z.string(),
	method: z.string(),
	debug: z.object({
		duration: z.string(),
	}),
});

// Schema for validating the JSON parsed from the HTML of the stats page.
const stats_schema = z.object({
	chapters: z.array(
		z.object({
			date: z.string(),
			title: z.string(),
			views: z.string(),
			words: z.string(),
			words_text: z.string(),
			chapter_num: z.number(),
		}),
	),
	stats: z.object({
		data: z.array(
			z.object({
				views: z.number().optional(),
				likes: z.number().optional(),
				dislikes: z.number().optional(),
				date: z.string(),
			}),
		),
		first_chapter_date: z.string(),
		last_chapter_date: z.string(),
	}),
});

type Tag = {
	id: number;
	title: string;
	type: string;
	href: string;
	text: string;
};

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

		// Check for rate limiting.
		if (api_status === 429) {
			await sleep(start_time, Date.now(), 5000);
			id = id - 1;
			continue;
		}

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
