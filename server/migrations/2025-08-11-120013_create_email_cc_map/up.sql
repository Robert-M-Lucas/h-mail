CREATE TABLE IF NOT EXISTS EmailCcMap (
    email_id INTEGER NOT NULL ,
    email TEXT NOT NULL,
    name TEXT,
    FOREIGN KEY (email_id) REFERENCES Emails(email_id) ON DELETE CASCADE
)