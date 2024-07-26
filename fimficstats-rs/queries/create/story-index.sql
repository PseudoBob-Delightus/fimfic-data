CREATE TABLE IF NOT EXISTS Story_index (
	story_id        integer NOT NULL PRIMARY KEY,
	status          text    NOT NULL,
	version         integer NOT NULL,
	timestamp       integer NOT NULL,
	api_time        integer,
	story_page_time integer,
	stats_page_time integer
);
