CREATE TABLE IF NOT EXISTS Tags (
	id    integer NOT NULL PRIMARY KEY,
	title text    NOT NULL,
	TYPE  text    NOT NULL,
	text  text    NOT NULL,
	href  text    NOT NULL
)
