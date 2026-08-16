CREATE TABLE IF NOT EXISTS Tags (
	id    integer NOT NULL PRIMARY KEY,
	title text    NOT NULL,
	type  text    NOT NULL,
	text  text    NOT NULL,
	href  text    NOT NULL
);
