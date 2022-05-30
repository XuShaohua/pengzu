-- Your SQL goes here

-- CREATE TABLE authors (
--     id INTEGER PRIMARY KEY,
--     name TEXT NOT NULL COLLATE NOCASE,
--     sort TEXT COLLATE NOCASE,
--     link TEXT NOT NULL DEFAULT "",
--     UNIQUE (name)
-- )

CREATE TABLE authors
(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    sort TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name)
)
