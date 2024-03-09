export const story_index_table = `CREATE TABLE IF NOT EXISTS Story_index (
	story_id            integer     PRIMARY KEY,
	status              text        NOT NULL,
	version             integer     NOT NULL,
	timestamp           integer     NOT NULL
)`;

export const authors_table = `CREATE TABLE IF NOT EXISTS Authors (
	id                  integer     PRIMARY KEY,
	name                text        NOT NULL,
	followers           integer     NOT NULL,
	blogs               integer     NOT NULL
)`;

export const stories_table = `CREATE TABLE IF NOT EXISTS Stories (
	id                  integer     PRIMARY KEY,
	title               text        NOT NULL,
	date_modified       integer     NOT NULL,
	date_updated        integer     NOT NULL,
	date_published      integer     NOT NULL,
	cover               integer     NOT NULL,
	color_hex           integer     NOT NULL,
	views               integer     NOT NULL,
	total_views         integer     NOT NULL,
	num_comments        integer     NOT NULL,
	rating              integer     NOT NULL,
	completion_status   text        NOT NULL,
	content_rating      text        NOT NULL,
	likes               integer     NOT NULL,
	dislikes            integer     NOT NULL,
	author_id           integer     NOT NULL,
	prequel_id          integer,

	CONSTRAINT stories_author_id_fk FOREIGN KEY (author_id)
   	REFERENCES Authors (id),
	
	CONSTRAINT story_index_id_fk FOREIGN KEY (id)
   	REFERENCES Story_index (story_id)
)`;

export const tags_table = `CREATE TABLE IF NOT EXISTS Tags (
	id                  integer     PRIMARY KEY,
	title               text        NOT NULL,
	type                text        NOT NULL,
	text                text        NOT NULL,
	href                text        NOT NULL
)`;

export const tag_links_table = `CREATE TABLE IF NOT EXISTS Tags_links (
	story_id            integer,
	tag_id              integer,

	CONSTRAINT tag_links_story_id_fk FOREIGN KEY (story_id)
		REFERENCES Stories (id),
	CONSTRAINT tag_links_tag_id_fk FOREIGN KEY (tag_id)
		REFERENCES Tags (id),

	CONSTRAINT tag_links_pk PRIMARY KEY (story_id, tag_id)
)`;

export const chapters_table = `CREATE TABLE IF NOT EXISTS Chapters (
	story_id            integer     NOT NULL,
	chapter_num         integer     NOT NULL,
	id                  integer,
	title               text        NOT NULL,
	date_modified       integer     NOT NULL,
	views               integer     NOT NULL,
	words               integer     NOT NULL,

	CONSTRAINT chapter_story_id_fk FOREIGN KEY (story_id)
   	REFERENCES Stories (id),
	
	CONSTRAINT chapters_pk PRIMARY KEY (story_id, chapter_num)
)`;

export const stats_table = `CREATE TABLE IF NOT EXISTS Stats (
	story_id            integer     NOT NULL,
	date                integer     NOT NULL,
	views               integer,
	likes               integer,
	dislikes            integer,

	CONSTRAINT stats_story_id_fk FOREIGN KEY (story_id)
   	REFERENCES Stories (id),
	
	CONSTRAINT stats_pk PRIMARY KEY (story_id, date)
)`;

export const referral_sites_table = `CREATE TABLE IF NOT EXISTS Referral_sites (
	id                  integer     PRIMARY KEY,
	site                string      NOT NULL
)`;

export const referrals_table = `CREATE TABLE IF NOT EXISTS Referrals (
	story_id            integer     NOT NULL,
	referral_site_id    integer     NOT NULL,
	count               integer     NOT NULL,

	CONSTRAINT referrals_story_id_fk FOREIGN KEY (story_id)
   	REFERENCES Stories (id),
	
	CONSTRAINT referrals_referral_site_id_fk FOREIGN KEY (referral_site_id)
   	REFERENCES Referral_sites (id),
	
	CONSTRAINT referrals_pk PRIMARY KEY (story_id, referral_site_id)
)`;