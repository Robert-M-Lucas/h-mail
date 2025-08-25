CREATE TABLE IF NOT EXISTS user_whitelists (
    user_id BIGINT NOT NULL,
    address TEXT NOT NULL,
    place_in TEXT NOT NULL CHECK (place_in IN ('MINIMUM', 'ACCEPTED', 'PERSONAL')),
    PRIMARY KEY (user_id, address),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);
