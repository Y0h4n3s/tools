CREATE TABLE IF NOT EXISTS end_points (
    id SERIAL PRIMARY KEY,
    date_added TIMESTAMP NOT NULL DEFAULT NOW(),
    list_type CHAR NOT NULL DEFAULT 'd',
    href TEXT DEFAULT '',
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
    path_href TEXT DEFAULT '',
    link_from TEXT DEFAULT '',
    hitcount INT NOT NULL,
    eid INT,
    CONSTRAINT eid
        FOREIGN KEY(eid)
            REFERENCES end_points(id)
);

CREATE TABLE IF NOT EXISTS params (
    id SERIAL PRIMARY KEY,
    key TEXT,
    value TEXT DEFAULT '',

    epid int REFERENCES end_point(id)
)
