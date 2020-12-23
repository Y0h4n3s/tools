CREATE TABLE IF NOT EXISTS dump_collector (
    id SERIAL PRIMARY KEY,
    hostname TEXT DEFAULT '',
    full_path TEXT DEFAULT '',
    protocol TEXT DEFAULT '',
    path_only TEXT DEFAULT '',
    full_params TEXT DEFAULT '',
    href TEXT DEFAULT '',
    path_href TEXT DEFAULT '',
    link_from TEXT DEFAULT '',
    ip TEXT DEFAULT '',
    port INT DEFAULT 0
)