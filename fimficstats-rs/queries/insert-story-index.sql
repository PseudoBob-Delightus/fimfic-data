INSERT INTO
	Story_index (story_id, STATUS, version, timestamp)
VALUES
	(?1, ?2, ?3, ?4)
ON CONFLICT(story_id) DO UPDATE SET
	STATUS = excluded.status,
	version = excluded.version,
	timestamp = excluded.timestamp;
