-- Your SQL goes here
-- CREATE TABLE series ( id   INTEGER PRIMARY KEY,
--                       name TEXT NOT NULL COLLATE NOCASE,
--                       sort TEXT COLLATE NOCASE,
--                       UNIQUE (name)
-- )

CREATE TABLE series (
    id   SERIAL PRIMARY KEY NOT NULL ,
    name TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name)
)