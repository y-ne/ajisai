CREATE TABLE IF NOT EXISTS key_value (
    id BIGSERIAL PRIMARY KEY,
    key TEXT NOT NULL,
    value JSONB NOT NULL,
    ttl TIMESTAMPTZ
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_key_value_key ON key_value (key);
