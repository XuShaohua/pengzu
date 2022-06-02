-- Your SQL goes here

CREATE TABLE import_projects
(
    id SERIAL PRIMARY KEY,
    calibre_path TEXT NOT NULL,
    status INTEGER NOT NULL,
    total INTEGER NOT NULL,
    worker_pid INTEGER,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)