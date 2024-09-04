CREATE TABLE IF NOT EXISTS Story_updates (
	id                integer NOT NULL,
	loop_index        integer NOT NULL,
	date_modified     integer NOT NULL,
	date_updated      integer NOT NULL,
	date_published    integer NOT NULL,
	views             integer NOT NULL,
	total_views       integer NOT NULL,
	num_comments      integer NOT NULL,
	rating            integer NOT NULL,
	completion_status text    NOT NULL,
	likes             integer NOT NULL,
	dislikes          integer NOT NULL,
	prequel_id        integer,

		CONSTRAINT stories_author_id_fk FOREIGN KEY (author_id)
			REFERENCES Authors (id),

		CONSTRAINT story_index_id_fk FOREIGN KEY (id)
			REFERENCES Story_index (story_id),

		CONSTRAINT story_updates_pk PRIMARY KEY (story_id, loop_index)
)
