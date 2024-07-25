INSERT INTO
	Chapters (
		story_id,
		chapter_num,
		title,
		date_modified,
		views,
		words
	)
VALUES
	(?1, ?2, ?3, ?4, ?5, ?6)
ON CONFLICT(story_id, chapter_num) DO UPDATE SET
	title = excluded.title,
	date_modified = excluded.date_modified,
	views = excluded.views,
	words = excluded.words;
