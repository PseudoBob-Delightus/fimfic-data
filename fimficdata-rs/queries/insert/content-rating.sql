INSERT INTO
	Content_rating (id, rating)
VALUES
	(?1, ?2)
ON CONFLICT(id) DO UPDATE SET
	type = excluded.rating;
