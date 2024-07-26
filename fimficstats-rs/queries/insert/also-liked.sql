INSERT INTO Also_liked (story_id, also_liked_id) 
	VALUES (?1, ?2)
ON CONFLICT(story_id, also_liked_id) DO UPDATE SET
	story_id = excluded.story_id,
	also_liked_id = excluded.also_liked_id;
