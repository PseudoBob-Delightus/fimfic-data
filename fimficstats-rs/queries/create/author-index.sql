CREATE TABLE IF NOT EXISTS Author_index (
	id           integer NOT NULL PRIMARY KEY,
	name         text    NOT NULL,
	bio_length   integer NOT NULL,
	date_joined  integer NOT NULL,
	avatar       integer NOT NULL
);
