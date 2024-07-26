CREATE TABLE IF NOT EXISTS Stats (
	story_id INTEGER NOT NULL,
	date     INTEGER NOT NULL,
	views    INTEGER,
	likes    INTEGER,
	dislikes INTEGER,

		CONSTRAINT stats_story_id_fk FOREIGN KEY (story_id)
			REFERENCES Stories (id),
	
		CONSTRAINT stats_pk PRIMARY KEY (story_id, date)
);
