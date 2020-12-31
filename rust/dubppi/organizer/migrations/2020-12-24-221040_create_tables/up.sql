CREATE TABLE IF NOT EXISTS sub_domains (
    id SERIAL PRIMARY KEY,
    hostname TEXT,
    is_root BOOL DEFAULT FALSE,
    ip TEXT DEFAULT '',
    vhost BOOL DEFAULT FALSE,
    date_added TIMESTAMP NOT NULL DEFAULT NOW(),
    notes TEXT DEFAULT '',
    port INT DEFAULT 443,
    protocol TEXT DEFAULT 'https'
);

CREATE TABLE IF NOT EXISTS end_points (
    id SERIAL PRIMARY KEY,
    date_added TIMESTAMP NOT NULL DEFAULT NOW(),
    list_type CHAR NOT NULL DEFAULT 'n',
    href TEXT DEFAULT '',
    port INT DEFAULT 443,
    protocol TEXT DEFAULT 'https',
    sid INT,
    CONSTRAINT sid
        FOREIGN KEY(sid)
            REFERENCES sub_domains(id)
            ON DELETE CASCADE
    );

CREATE TABLE IF NOT EXISTS end_point (
    id SERIAL PRIMARY KEY,
    value TEXT,
    href TEXT DEFAULT '',
    path_only TEXT DEFAULT '',
    link_from TEXT DEFAULT '',
    hitcount INT NOT NULL,
    full_path TEXT DEFAULT '',
    params TEXT DEFAULT '',
    eid INT,
    CONSTRAINT eid
        FOREIGN KEY(eid)
            REFERENCES end_points(id)
);

CREATE TABLE IF NOT EXISTS dom_xss (
    id SERIAL PRIMARY KEY,
    kind TEXT NOT NULL,
    link_from TEXT DEFAULT '',
    hostname TEXT DEFAULT '',
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS configs (
    id SERIAL PRIMARY KEY,
    key TEXT NOT NULL,
    value TEXT DEFAULT ''
);
