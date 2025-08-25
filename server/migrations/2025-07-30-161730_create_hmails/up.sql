CREATE TABLE IF NOT EXISTS hmails (
    hmail_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id BIGINT NOT NULL,
    outbox BOOLEAN NOT NULL,
    context_for BIGINT,
    sender TEXT NOT NULL,
    sender_name TEXT,  -- Nullable
    subject TEXT NOT NULL,
    sent_at BIGINT NOT NULL,
    received_at BIGINT NOT NULL,
    random_id BIGINT NOT NULL,
    reply_to TEXT,  -- Nullable
    reply_to_name TEXT,
    parent TEXT,  -- Nullable
    body TEXT NOT NULL,
    hash TEXT NOT NULL,
    pow_classification TEXT NOT NULL CHECK (pow_classification IN ('MINIMUM', 'ACCEPTED', 'PERSONAL')),
    CONSTRAINT reply_to_name_check CHECK (reply_to IS NOT NULL OR reply_to_name IS NULL),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (context_for) REFERENCES hmails(hmail_id) ON DELETE CASCADE
);
