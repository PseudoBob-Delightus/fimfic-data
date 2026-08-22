INSERT INTO
	Completion_status (id, status)
VALUES
	(?1, ?2)
ON CONFLICT(id) DO UPDATE SET
	type = excluded.status;
