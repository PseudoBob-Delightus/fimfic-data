INSERT INTO
	Stories (
		id, title, date_modified, date_updated, date_published,
		cover, cover_source, color_hex, views, total_views,
		num_comments, featured, rating, completion_status, content_rating,
		likes, dislikes, ranking, word_ranking, bookshelves,
		tracking, groups, author_id, prequel_id
	)
VALUES
	(
		?1, ?2, ?3, ?4, ?5,
		?6, ?7, ?8, ?9, ?10,
		?11, ?12, ?13, ?14, ?15,
		?16, ?17, ?18, ?19, ?20,
		?21, ?22, ?23, ?24, ?25
	)
ON CONFLICT(id) DO UPDATE SET
	title = excluded.title,
	date_modified = excluded.date_modified,
	date_updated = excluded.date_updated,
	date_published = excluded.date_published,
	cover = excluded.cover,
	cover_source = excluded.cover_source,
	color_hex = excluded.color_hex,
	views = excluded.views,
	total_views = excluded.total_views,
	num_comments = excluded.num_comments,
	featured = excluded.featured,
	rating = excluded.rating,
	completion_status = excluded.completion_status,
	content_rating = excluded.content_rating,
	likes = excluded.likes,
	dislikes = excluded.dislikes,
	ranking = excluded.ranking,
	word_ranking = excluded.word_ranking,
	bookshelves = excluded.bookshelves,
	tracking = excluded.tracking,
	groups = excluded.groups,
	author_id = excluded.author_id,
	prequel_id = excluded.prequel_id;
