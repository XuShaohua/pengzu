-- Your SQL goes here

CREATE TABLE import_details
(
    id SERIAL PRIMARY KEY,
    project INTEGER NOT NULL,
    calibre_book INTEGER NOT NULL,
    status INTEGER NOT NULL,
    book INTEGER,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)