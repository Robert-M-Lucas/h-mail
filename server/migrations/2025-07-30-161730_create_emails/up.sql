CREATE TABLE IF NOT EXISTS Emails (
    email_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL ,
    user_id INTEGER NOT NULL,
    subject TEXT NOT NULL,
    sent_at INTEGER NOT NULL,
    received_at INTEGER NOT NULL,
    mime_version TEXT NOT NULL,
    content_type TEXT NOT NULL,
    reply_to TEXT, -- Nullable
    parent TEXT, -- Nullable
    hash TEXT NOT NULL,
    pow_classification TEXT CHECK(pow_classification IN ('MINIMUM', 'ACCEPTED', 'PERSONAL')) NOT NULL ,
    FOREIGN KEY (user_id) REFERENCES Users(user_id) ON DELETE CASCADE
)