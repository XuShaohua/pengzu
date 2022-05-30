-- Your SQL goes here

-- CREATE TABLE publishers (
--     id INTEGER PRIMARY KEY,
--     name TEXT NOT NULL COLLATE NOCASE,
--     sort TEXT COLLATE NOCASE,
--     UNIQUE (name)
-- )

CREATE TABLE publishers
(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    sort TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name)
)