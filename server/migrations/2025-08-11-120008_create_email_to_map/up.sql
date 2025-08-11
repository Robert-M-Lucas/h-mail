CREATE TABLE IF NOT EXISTS EmailToMap (
    email_id INTEGER NOT NULL ,
    email TEXT NOT NULL,
    name TEXT,
    FOREIGN KEY (email_id) REFERENCES Emails(email_id) ON DELETE CASCADE
)