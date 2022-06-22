-- Your SQL goes here

CREATE TABLE identifier_types
(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    url_template TEXT NOT NULL DEFAULT '',
    description TEXT,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name)
)