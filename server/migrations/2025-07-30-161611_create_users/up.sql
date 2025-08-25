CREATE TABLE IF NOT EXISTS users (
     user_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
     username TEXT UNIQUE NOT NULL,
     password_hash TEXT NOT NULL,
     pow_minimum INTEGER NOT NULL,
     pow_accepted INTEGER NOT NULL,
     pow_personal INTEGER NOT NULL
);
