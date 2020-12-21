
CREATE TABLE IF NOT EXISTS sub_domains (
    id SERIAL PRIMARY KEY,
    hostname TEXT UNIQUE,
    is_root BOOL DEFAULT FALSE,
    ip TEXT DEFAULT '',
    vhost BOOL DEFAULT FALSE,
    date_added TIMESTAMP NOT NULL DEFAULT NOW(),
    notes TEXT DEFAULT '',
    protocol TEXT DEFAULT 'HTTP'
);
    
CREATE TABLE IF NOT EXISTS end_points (
    id SERIAL PRIMARY KEY,
    date_added TIMESTAMP NOT NULL DEFAULT NOW(),
    list_type CHAR NOT NULL DEFAULT 'd',
    sid TEXT,
    CONSTRAINT sid
        FOREIGN KEY(sid)
            REFERENCES sub_domains(hostname)
            ON DELETE CASCADE
    );

CREATE TABLE IF NOT EXISTS end_point (
    id SERIAL PRIMARY KEY,
    value TEXT,
    params TEXT,
    hitcount INT NOT NULL,
    eid INT,
    CONSTRAINT eid
        FOREIGN KEY(eid)
            REFERENCES end_points(id)
);

