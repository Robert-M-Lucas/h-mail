CREATE TABLE IF NOT EXISTS Users (
     user_id INTEGER PRIMARY KEY AUTOINCREMENT,
     username TEXT UNIQUE NOT NULL,
     password_hash TEXT NOT NULL,
     pow_minimum INTEGER NOT NULL,
     pow_accepted INTEGER NOT NULL,
     pow_personal INTEGER NOT NULL
)