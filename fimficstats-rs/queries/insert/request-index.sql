INSERT INTO Request_index (
	request_type_id, loop_index, version,
	timestamp, api_duration_sec, round_trip_time_ms,
	stories_requested, stories_returned,
	tags_returned, authors_returned, total_stories
) VALUES (
	?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11
);
