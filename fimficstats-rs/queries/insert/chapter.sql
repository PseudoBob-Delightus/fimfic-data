INSERT OR IGNORE INTO
	Chapters (
		story_id,
		chapter_num,
		title,
		views,
		words,
		date_published
	)
VALUES
	(?1, ?2, ?3, ?4, ?5, ?6);
