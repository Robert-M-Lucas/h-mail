CREATE TABLE IF NOT EXISTS Emails (
    email_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL ,
    user_id INTEGER NOT NULL,
    source TEXT NOT NULL,
    subject TEXT NOT NULL,
    sent_at BIGINT NOT NULL,
    received_at BIGINT NOT NULL,
    reply_to TEXT, -- Nullable
    reply_to_name TEXT CHECK (reply_to IS NOT NULL OR reply_to_name IS NULL), -- Can only be set if reply_to is set
    parent TEXT, -- Nullable
    body TEXT NOT NULL,
    hash TEXT NOT NULL,
    pow_classification TEXT NOT NULL CHECK(pow_classification IN ('MINIMUM', 'ACCEPTED', 'PERSONAL')),
    FOREIGN KEY (user_id) REFERENCES Users(user_id) ON DELETE CASCADE
)