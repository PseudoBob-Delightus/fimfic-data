CREATE TABLE IF NOT EXISTS Story_index (
	story_id  integer NOT NULL PRIMARY KEY,
	STATUS    text    NOT NULL,
	version   integer NOT NULL,
	timestamp integer NOT NULL
);
