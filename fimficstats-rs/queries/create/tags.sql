CREATE TABLE IF NOT EXISTS Tags (
	id     integer NOT NULL PRIMARY KEY,
	name   text    NOT NULL,
	type   text    NOT NULL,
	href   text    NOT NULL,
	old_id text
);
