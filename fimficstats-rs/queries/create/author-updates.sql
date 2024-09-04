CREATE TABLE IF NOT EXISTS Author_updates (
	index      integer PRIMARY KEY AUTOINCREMENT,
	author_id  integer NOT NULL,
	loop_index integer NOT NULL,
	followers  integer NOT NULL,
	stories    integer NOT NULL,
	blogs      integer NOT NULL,

		CONSTRAINT author_updates_authors_fk FOREIGN KEY (author_id)
			REFERENCES Authors (id)
);
