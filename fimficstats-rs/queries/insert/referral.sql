INSERT INTO Referrals (
	story_id, referral_site_id, count
) VALUES (?1, ?2, ?3)
ON CONFLICT(story_id, referral_site_id) DO UPDATE SET
	count = excluded.count;
