INSERT INTO
	Authors (id, name, date_joined, followers, blogs)
VALUES
	(?1, ?2, ?3, ?4, ?5)
ON CONFLICT(id) DO UPDATE SET
	name = excluded.name,
	date_joined = excluded.date_joined,
	followers = excluded.followers,
	blogs = excluded.blogs;
