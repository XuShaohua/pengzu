-- Your SQL goes here

-- CREATE TABLE data (
--     id INTEGER PRIMARY KEY,
--     book INTEGER NOT NULL,
--     format TEXT NOT NULL COLLATE NOCASE,
--     uncompressed_size INTEGER NOT NULL,
--     name TEXT NOT NULL,
--     UNIQUE (book, format)
-- )

CREATE TABLE data
(
    id SERIAL PRIMARY KEY,
    book INTEGER NOT NULL,
    format TEXT NOT NULL,
    uncompressed_size INTEGER NOT NULL,
    name TEXT NOT NULL,
    sha TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book, format)
)
