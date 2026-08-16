CREATE TABLE IF NOT EXISTS Chapters (
	story_id       integer NOT NULL,
	chapter_num    integer NOT NULL,
	title          text    NOT NULL,
	views          integer NOT NULL,
	words          integer NOT NULL,
	date_published integer NOT NULL,

		CONSTRAINT chapter_story_id_fk FOREIGN KEY (story_id)
			REFERENCES Stat_pages (story_id),

		CONSTRAINT chapters_pk PRIMARY KEY (story_id, chapter_num)
);
