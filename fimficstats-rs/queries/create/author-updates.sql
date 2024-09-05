CREATE TABLE IF NOT EXISTS Author_updates (
	index      integer PRIMARY KEY autoincrement,
	author_id  integer NOT NULL,
	loop_id    integer NOT NULL,
	followers  integer NOT NULL,
	stories    integer NOT NULL,
	blogs      integer NOT NULL,

	CONSTRAINT author_updates_author_id_fk FOREIGN KEY (author_id)
		REFERENCES Authors (id),

	CONSTRAINT author_updates_loop_id_fk FOREIGN KEY (loop_id)
		REFERENCES Loops (id)
);
