CREATE TABLE IF NOT EXISTS UserWhitelists (
    user_id INTEGER NOT NULL,
    whitelisted TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES Users(user_id) ON DELETE CASCADE
)