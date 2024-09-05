CREATE TABLE IF NOT EXISTS Featured_stories (
	story_id   integer NOT NULL PRIMARY KEY,
	loop_index integer NOT NULL,

	CONSTRAINT featured_stories_story_id_fk FOREIGN KEY (story_id)
		REFERENCES Stories (id)
);
