CREATE TABLE IF NOT EXISTS UserWhitelists (
    user_id INTEGER NOT NULL,
    address TEXT NOT NULL,
    place_in TEXT NOT NULL CHECK(place_in IN ('MINIMUM', 'ACCEPTED', 'PERSONAL')),
    FOREIGN KEY (user_id) REFERENCES Users(user_id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, address)
)