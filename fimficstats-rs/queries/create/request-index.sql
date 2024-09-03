CREATE TABLE IF NOT EXISTS Request_index (
	id                 integer primary key autoincrement,
	request_type_id    integer not null,
	loop_index         integer not null,
	version            integer not null,
	timestamp          integer not null,
	api_duration_sec   integer not null,
	round_trip_time_ms integer not null,
	stories_requested  integer not null,
	stories_returned   integer not null,
	tags_returned      integer not null,
	authors_returned   integer not null,
	total_stories      integer not null
);
