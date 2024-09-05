CREATE TABLE IF NOT EXISTS Tag_links (
	story_id   integer NOT NULL,
	tag_id     integer NOT NULL,
	loop_index integer NOT NULL,

	CONSTRAINT tag_links_story_id_fk FOREIGN KEY (story_id)
		REFERENCES Stories (id),

	CONSTRAINT tag_links_tag_id_fk FOREIGN KEY (tag_id)
		REFERENCES Tags (id),

	CONSTRAINT tag_links_pk PRIMARY KEY (story_id, tag_id, loop_index)
);
