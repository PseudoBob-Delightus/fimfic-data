CREATE TABLE IF NOT EXISTS Tag_links (
	story_id INTEGER,
	tag_id   INTEGER,

		CONSTRAINT tag_links_story_id_fk FOREIGN KEY (story_id)
			REFERENCES Stories (id),

		CONSTRAINT tag_links_tag_id_fk FOREIGN KEY (tag_id)
			REFERENCES Tags (id),

		CONSTRAINT tag_links_pk PRIMARY KEY (story_id, tag_id)
);
