CREATE TABLE IF NOT EXISTS dns_names(
    id SERIAL PRIMARY KEY,
    a TEXT,
    mx TEXT,
    sid TEXT REFERENCES sub_domains(hostname)
)