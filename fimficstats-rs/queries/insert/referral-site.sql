INSERT INTO Referral_sites (site) 
	VALUES (?1)
ON CONFLICT(site) DO UPDATE SET
	site = excluded.site;
