CREATE TABLE IF NOT EXISTS Stories (
	id                integer NOT NULL PRIMARY KEY,
	title             text    NOT NULL,
	date_modified     integer NOT NULL,
	date_updated      integer NOT NULL,
	date_published    integer NOT NULL,
	cover             integer NOT NULL,
	cover_source      text    NOT NULL,
	color_hex         integer NOT NULL,
	views             integer NOT NULL,
	total_views       integer NOT NULL,
	num_comments      integer NOT NULL,
	featured          integer NOT NULL,
	rating            integer NOT NULL,
	completion_status text    NOT NULL,
	content_rating    text    NOT NULL,
	likes             integer NOT NULL,
	dislikes          integer NOT NULL,
	ranking           integer NOT NULL,
	word_ranking      integer NOT NULL,
	bookshelves       integer NOT NULL,
	tracking          integer NOT NULL,
	groups            integer NOT NULL,
	author_id         integer NOT NULL,
	prequel_id        integer,

		CONSTRAINT stories_author_id_fk FOREIGN KEY (author_id)
			REFERENCES Authors (id),

		CONSTRAINT story_index_id_fk FOREIGN KEY (id)
			REFERENCES Story_index (story_id)
)
