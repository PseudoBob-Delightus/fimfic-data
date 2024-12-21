INSERT INTO
	Request_type (id, type)
VALUES
	(?1, ?2)
ON CONFLICT(id) DO UPDATE SET
	type = excluded.type;
