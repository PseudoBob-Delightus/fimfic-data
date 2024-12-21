CREATE TABLE IF NOT EXISTS Stories (
	id                integer NOT NULL PRIMARY KEY,
	title             text    NOT NULL,
	content_rating_id integer NOT NULL,
	cover             integer NOT NULL,
	prequel_id        integer,
	author_id         integer NOT NULL,

	CONSTRAINT stories_content_rating_id_fk FOREIGN KEY (content_rating_id)
		REFERENCES Content_rating (id),

	CONSTRAINT stories_author_id_fk FOREIGN KEY (author_id)
		REFERENCES Authors (id)
);
