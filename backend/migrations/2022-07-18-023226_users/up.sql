-- Your SQL goes here

CREATE TABLE users
(
    id   SERIAL PRIMARY KEY NOT NULL ,
    name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    email TEXT NOT NULL,
    role INTEGER NOT NULL,
    salt TEXT NOT NULL,
    hash TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    UNIQUE (name),
    UNIQUE (email)
)