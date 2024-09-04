CREATE TABLE IF NOT EXISTS Author_updates (
	id         integer NOT NULL,
	loop_index integer NOT NULL,
	followers  integer NOT NULL,
	stories    integer NOT NULL,
	blogs      integer NOT NULL,

		CONSTRAINT author_updates_authors_fk FOREIGN KEY (id)
			REFERENCES Authors (id),

		CONSTRAINT author_updates_pk PRIMARY KEY (id, loop_index)
);
