CREATE TABLE IF NOT EXISTS Referral_sites (
	id   integer NOT NULL PRIMARY KEY,
	site text    NOT NULL,

	UNIQUE(site)
);
