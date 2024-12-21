INSERT INTO Tags (id, title, TYPE, text, href)
VALUES
    (?1, ?2, ?3, ?4, ?5)
ON CONFLICT(id) DO UPDATE SET
	title = excluded.title,
	TYPE = excluded.type,
	text = excluded.text,
	href = excluded.href;
