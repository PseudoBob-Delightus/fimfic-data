INSERT OR IGNORE INTO
	Stat_pages (
		story_id, title, short_description, date_published, first_chapter_date, last_chapter_date,
		author_id, author_name, author_image_stub, author_bio, author_stories, author_blogs, author_followers,
		views, comments, ranking, word_ranking, bookshelves, tracking,
		timestamp, page_time, users_online, hits_today, hits_yesterday
	)
VALUES
	(
		?1, ?2, ?3, ?4, ?5, ?6,
		?7, ?8, ?9, ?10, ?11, ?12, ?13,
		?14, ?15, ?16, ?17, ?18, ?19,
		?20, ?21, ?22, ?23, ?24
	);
