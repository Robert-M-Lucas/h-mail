CREATE TABLE IF NOT EXISTS Emails (
    email_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL ,
    user_id INTEGER NOT NULL,
    source TEXT NOT NULL,
    email TEXT NOT NULL,
    pow_classification TEXT CHECK(pow_classification IN ('MINIMUM', 'ACCEPTED', 'PERSONAL')) NOT NULL ,
    FOREIGN KEY (user_id) REFERENCES Users(user_id) ON DELETE CASCADE
)