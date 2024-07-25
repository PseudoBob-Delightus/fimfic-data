INSERT INTO
	Tag_links (story_id, tag_id)
VALUES
	(?1, ?2)
ON CONFLICT(story_id, tag_id) DO UPDATE SET
	story_id = excluded.story_id,
	tag_id = excluded.tag_id;
