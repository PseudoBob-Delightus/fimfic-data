CREATE TABLE IF NOT EXISTS Requests (
	id                 integer PRIMARY KEY autoincrement,
	request_type_id    integer NOT NULL,
	loop_index         integer NOT NULL,
	version            integer NOT NULL,
	timestamp          integer NOT NULL,
	api_duration_sec   integer NOT NULL,
	round_trip_time_ms integer NOT NULL,
	stories_requested  integer NOT NULL,
	stories_returned   integer NOT NULL,
	tags_returned      integer NOT NULL,
	authors_returned   integer NOT NULL,
	total_stories      integer NOT NULL,

	CONSTRAINT requests_request_type_id_fk FOREIGN KEY (request_type_id)
		REFERENCES Request_type (id),
);
