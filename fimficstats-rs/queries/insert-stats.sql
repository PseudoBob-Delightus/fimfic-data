INSERT INTO
	Stats (story_id, date, views, likes, dislikes)
VALUES
	(?1, ?2, ?3, ?4, ?5)
ON CONFLICT(story_id, date) DO UPDATE SET
	views = excluded.views,
	likes = excluded.likes,
	dislikes = excluded.dislikes;
