CREATE TABLE IF NOT EXISTS Stat_pages (
	-- story data
	story_id           integer NOT NULL PRIMARY KEY,
	title              text    NOT NULL,
	short_description  text    NOT NULL,
	date_published     text    NULL,
	first_chapter_date integer NULL,
	last_chapter_date  integer NULL,

	-- author data
	author_id         integer NOT NULL,
	author_name       text    NOT NULL,
	author_image_stub text    NOT NULL,
	author_bio        text    NULL,
	author_stories    integer NOT NULL,
	author_blogs      integer NOT NULL,
	author_followers  integer NOT NULL,

	-- sidebar data
	views        integer NOT NULL,
	comments     integer NOT NULL,
	ranking      integer NOT NULL,
	word_ranking integer NOT NULL,
	bookshelves  integer NOT NULL,
	tracking     integer NOT NULL,

	-- page data
	timestamp      integer NOT NULL,
	page_time      float   NOT NULL,
	users_online   integer NOT NULL,
	hits_today     integer NOT NULL,
	hits_yesterday integer NOT NULL
)
