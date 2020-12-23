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

CREATE TABLE IF NOT EXISTS params (
    id SERIAL PRIMARY KEY,
    key TEXT,
    value TEXT,
    epid int REFERENCES end_point(id)
)
