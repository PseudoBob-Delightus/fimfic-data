CREATE TABLE IF NOT EXISTS Stats (
	story_id integer NOT NULL,
	views    integer,
	likes    integer,
	dislikes integer,
	date     integer NOT NULL,

		CONSTRAINT stats_story_id_fk FOREIGN KEY (story_id)
			REFERENCES Stat_pages (story_id),
	
		CONSTRAINT stats_pk PRIMARY KEY (story_id, date)
);
