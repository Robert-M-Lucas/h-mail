CREATE TABLE IF NOT EXISTS HmailCcMap (
    hmail_id INTEGER NOT NULL,
    address TEXT NOT NULL,
    username TEXT,
    FOREIGN KEY (hmail_id) REFERENCES Hmails(hmail_id) ON DELETE CASCADE,
    PRIMARY KEY (hmail_id, address)
)