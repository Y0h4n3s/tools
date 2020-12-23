

CREATE TABLE IF NOT EXISTS sub_domains (
    id SERIAL PRIMARY KEY,
    hostname TEXT,
    is_root BOOL DEFAULT FALSE,
    ip TEXT DEFAULT '',
    vhost BOOL DEFAULT FALSE,
    date_added TIMESTAMP NOT NULL DEFAULT NOW(),
    notes TEXT DEFAULT '',
    port INT DEFAULT 0,
    protocol TEXT DEFAULT ''
);
