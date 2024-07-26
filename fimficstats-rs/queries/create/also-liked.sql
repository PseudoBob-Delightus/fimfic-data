CREATE TABLE IF NOT EXISTS Also_liked (
	story_id INTEGER,
	also_liked_id INTEGER,

	CONSTRAINT also_liked_story_id_fk FOREIGN KEY (story_id)
		REFERENCES Stories (id),

	CONSTRAINT also_liked_pk PRIMARY KEY (story_id, also_liked_id)
);
