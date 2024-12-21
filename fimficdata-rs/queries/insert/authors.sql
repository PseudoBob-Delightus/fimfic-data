INSERT INTO
	Authors (id, name, bio_length, date_joined, avatar)
VALUES
	(?1, ?2, ?3, ?4, ?5)
ON CONFLICT(id) DO UPDATE SET
	name = excluded.name,
	bio_length = excluded.bio_length,
	date_joined = excluded.date_joined,
	avatar = excluded.avatar;
