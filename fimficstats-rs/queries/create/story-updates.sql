CREATE TABLE IF NOT EXISTS Story_updates (
	id                    integer PRIMARY KEY autoincrement,
	story_id              integer NOT NULL,
	loop_index            integer NOT NULL,
	date_modified         integer NOT NULL,
	date_updated          integer NOT NULL,
	date_published        integer NOT NULL,
	views                 integer NOT NULL,
	total_views           integer NOT NULL,
	num_comments          integer NOT NULL,
	rating                integer NOT NULL,
	completion_status_id  integer NOT NULL,
	likes                 integer NOT NULL,
	dislikes              integer NOT NULL,

	CONSTRAINT story_updates_story_id_fk FOREIGN KEY (id)
		REFERENCES Stories (id),
	
	CONSTRAINT story_updates_completion_status_id_fk FOREIGN KEY (id)
		REFERENCES Completion_status (id)
)
