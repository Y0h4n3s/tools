
CREATE TABLE IF NOT EXISTS port_protocol (
    pid INT PRIMARY KEY,
    port INT NOT NULL CHECK (port >= 1 AND port <=65535),
    protocol VARCHAR(16) NOT NULL
    );


CREATE TABLE IF NOT EXISTS root_domains (
    rid INT PRIMARY KEY,
    hostname VARCHAR(256) UNIQUE,
    ip VARCHAR(32),
    vhost BOOL,
    date_added TIMESTAMP NOT NULL DEFAULT NOW(),
    notes VARCHAR(1024),
    pid INT,
        FOREIGN KEY(pid)
            REFERENCES port_protocol(pid)
            ON DELETE CASCADE
    );
    
    
CREATE TABLE IF NOT EXISTS sub_domains (
    sid INT PRIMARY KEY,
    hostname VARCHAR(256) UNIQUE,
    ip VARCHAR(32),
    vhost BOOL,
    date_added TIMESTAMP NOT NULL DEFAULT NOW(),
    notes VARCHAR(1024),
    pid INT,
    rid INT,
    CONSTRAINT pid
        FOREIGN KEY(pid)
            REFERENCES port_protocol(pid)
            ON DELETE CASCADE,
    CONSTRAINT rid
        FOREIGN KEY(rid)
            REFERENCES root_domains(rid)
            ON DELETE CASCADE
    );
    
CREATE TABLE IF NOT EXISTS endpoints (
    eid INT PRIMARY KEY,
    date_added TIMESTAMP NOT NULL DEFAULT NOW(),
    list_type CHAR NOT NULL DEFAULT 'd',
    rid INT,
    sid INT,
    CONSTRAINT rid
        FOREIGN KEY(rid)
            REFERENCES root_domains(rid)
            ON DELETE CASCADE,
    CONSTRAINT sid
        FOREIGN KEY(sid)
            REFERENCES sub_domains(sid)
            ON DELETE CASCADE
    );

CREATE TABLE IF NOT EXISTS endpoint (
    epid INT PRIMARY KEY,
    value VARCHAR(128),
    eid INT,
    CONSTRAINT eid
        FOREIGN KEY(eid)
            REFERENCES endpoints(eid)
);

CREATE TABLE IF NOT EXISTS params (
    pmid int PRIMARY KEY,
    type VARCHAR(32),
    parameter_name VARCHAR(64),
    epid INT,
    CONSTRAINT epid
        FOREIGN KEY(epid)
            REFERENCES endpoint(epid)
)