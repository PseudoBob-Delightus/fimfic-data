CREATE TABLE IF NOT EXISTS Authors (
	id           integer NOT NULL PRIMARY KEY,
	name         text    NOT NULL,
	date_joined  integer NOT NULL,
	followers    integer NOT NULL,
	blogs        integer NOT NULL
)
