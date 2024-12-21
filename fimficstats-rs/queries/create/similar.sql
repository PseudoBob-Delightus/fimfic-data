CREATE TABLE IF NOT EXISTS Similar (
	story_id   INTEGER,
	similar_id INTEGER,

	CONSTRAINT similar_story_id_fk FOREIGN KEY (story_id)
		REFERENCES Stories (id),

	CONSTRAINT similar_pk PRIMARY KEY (story_id, similar_id)
);
