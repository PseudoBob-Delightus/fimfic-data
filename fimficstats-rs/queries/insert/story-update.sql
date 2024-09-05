INSERT INTO
	Story_updates (
		story_id, loop_id, date_modified, date_updated, 
		date_published, views, total_views, num_comments, 
		rating, completion_status_id, likes, dislikes
	)
VALUES
	(
		?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12
	)
ON CONFLICT(id) DO UPDATE SET
	story_id = excluded.story_id,
	loop_id = excluded.loop_id,
	date_modified = excluded.date_modified,
	date_updated = excluded.date_updated,
	date_published = excluded.date_published,
	views = excluded.views,
	total_views = excluded.total_views,
	num_comments = excluded.num_comments,
	rating = excluded.rating,
	completion_status_id = excluded.completion_status_id,
	likes = excluded.likes,
	dislikes = excluded.dislikes;
