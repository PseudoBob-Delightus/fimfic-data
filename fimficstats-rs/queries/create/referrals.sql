CREATE TABLE IF NOT EXISTS Referrals (
	story_id         integer NOT NULL,
	referral_site_id integer NOT NULL,
	count            integer NOT NULL,

	CONSTRAINT referrals_story_id_fk FOREIGN KEY (story_id)
		REFERENCES Stat_pages (story_id),

	CONSTRAINT referrals_referral_site_id_fk FOREIGN KEY (referral_site_id)
		REFERENCES Referral_sites (id),

	CONSTRAINT referrals_pk PRIMARY KEY (story_id, referral_site_id)
);
