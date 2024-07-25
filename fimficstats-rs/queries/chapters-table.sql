CREATE TABLE IF NOT EXISTS Chapters (
	story_id      INTEGER NOT NULL,
	chapter_num   INTEGER NOT NULL,
	title         TEXT    NOT NULL,
	date_modified INTEGER NOT NULL,
	views         INTEGER NOT NULL,
	words         INTEGER NOT NULL,

		CONSTRAINT chapter_story_id_fk FOREIGN KEY (story_id)
			REFERENCES Stories (id),

		CONSTRAINT chapters_pk PRIMARY KEY (story_id, chapter_num)
);
