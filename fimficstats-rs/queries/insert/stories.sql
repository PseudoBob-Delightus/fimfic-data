INSERT INTO
	Stories (
		story_id, title, content_rating, cover, author_id
	)
VALUES
	(?1, ?2, ?3, ?4, ?5)
ON CONFLICT(story_id) DO UPDATE SET
	title = excluded.title,
	content_rating = excluded.content_rating,
	cover = excluded.cover,
	author_id = excluded.author_id;
