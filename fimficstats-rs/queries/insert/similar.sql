INSERT INTO Similar (story_id, similar_id) 
	VALUES (?1, ?2)
ON CONFLICT(story_id, similar_id) DO UPDATE SET
	story_id = excluded.story_id,
	similar_id = excluded.similar_id;
