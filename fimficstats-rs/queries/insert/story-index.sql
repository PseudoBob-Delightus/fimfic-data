INSERT INTO
	Story_index (
		story_id, status, version, timestamp, api_time, story_page_time, stats_page_time
	)
VALUES
	(?1, ?2, ?3, ?4, ?5, ?6, ?7)
ON CONFLICT(story_id) DO UPDATE SET
	status = excluded.status,
	version = excluded.version,
	timestamp = excluded.timestamp,
	api_time = excluded.api_time,
	story_page_time = excluded.story_page_time,
	stats_page_time = excluded.stats_page_time;
