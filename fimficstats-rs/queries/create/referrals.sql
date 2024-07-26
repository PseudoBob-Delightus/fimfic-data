CREATE TABLE IF NOT EXISTS Referrals (
	story_id         INTEGER NOT NULL,
	referral_site_id INTEGER NOT NULL,
	count            INTEGER NOT NULL,

	CONSTRAINT referrals_story_id_fk FOREIGN KEY (story_id)
		REFERENCES Stories (id),

	CONSTRAINT referrals_referral_site_id_fk FOREIGN KEY (referral_site_id)
		REFERENCES Referral_sites (id),

	CONSTRAINT referrals_pk PRIMARY KEY (story_id, referral_site_id)
);
