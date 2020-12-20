

CREATE TABLE IF NOT EXISTS root_domains (
    id SERIAL PRIMARY KEY,
    hostname VARCHAR(256) UNIQUE,
    ip VARCHAR(32) DEFAULT '',
    vhost BOOL DEFAULT FALSE,
    date_added TIMESTAMP NOT NULL DEFAULT NOW(),
    notes VARCHAR(1024) DEFAULT '',
    protocol VARCHAR(32) DEFAULT 'HTTP'
    );
    
    
CREATE TABLE IF NOT EXISTS sub_domains (
    id SERIAL PRIMARY KEY,
    hostname VARCHAR(256) UNIQUE,
    ip VARCHAR(32) DEFAULT '',
    vhost BOOL DEFAULT FALSE,
    date_added TIMESTAMP NOT NULL DEFAULT NOW(),
    notes VARCHAR(1024) DEFAULT '',
    protocol VARCHAR(32) DEFAULT 'HTTP',
    rid INT,
    CONSTRAINT rid
        FOREIGN KEY(rid)
            REFERENCES root_domains(id)
            ON DELETE CASCADE
    );
    
CREATE TABLE IF NOT EXISTS endpoints (
    id SERIAL PRIMARY KEY,
    date_added TIMESTAMP NOT NULL DEFAULT NOW(),
    list_type CHAR NOT NULL DEFAULT 'd',
    rid INT,
    sid INT,
    CONSTRAINT rid
        FOREIGN KEY(rid)
            REFERENCES root_domains(id)
            ON DELETE CASCADE,
    CONSTRAINT sid
        FOREIGN KEY(sid)
            REFERENCES sub_domains(id)
            ON DELETE CASCADE
    );

CREATE TABLE IF NOT EXISTS endpoint (
    id SERIAL PRIMARY KEY,
    value VARCHAR(128),
    params VARCHAR(512),
    eid INT,
    CONSTRAINT eid
        FOREIGN KEY(eid)
            REFERENCES endpoints(id)
);

