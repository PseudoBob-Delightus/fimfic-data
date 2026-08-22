INSERT INTO Tags (id, name, type, href, old_id)
VALUES
    (?1, ?2, ?3, ?4, ?5)
ON CONFLICT(id) DO UPDATE SET
	name = excluded.name,
	type = excluded.type,
	href = excluded.href,
	old_id = excluded.old_id;
