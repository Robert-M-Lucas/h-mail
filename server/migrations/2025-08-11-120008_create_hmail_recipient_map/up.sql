CREATE TABLE IF NOT EXISTS hmail_recipient_map (
      hmail_id BIGINT NOT NULL,
      address TEXT NOT NULL,
      username TEXT,
      PRIMARY KEY (hmail_id, address),
      FOREIGN KEY (hmail_id) REFERENCES hmails(hmail_id) ON DELETE CASCADE
);
