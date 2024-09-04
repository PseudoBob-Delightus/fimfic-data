CREATE TABLE IF NOT EXISTS Stories (
	id                integer NOT NULL PRIMARY KEY,
	title             text    NOT NULL,
	content_rating    text    NOT NULL,
	cover             integer NOT NULL,
	author_id         integer NOT NULL
);
