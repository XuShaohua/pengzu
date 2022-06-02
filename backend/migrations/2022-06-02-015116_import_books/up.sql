-- Your SQL goes here

CREATE TABLE import_books
(
    id SERIAL PRIMARY KEY,
    library INTEGER NOT NULL,
    calibre_book INTEGER NOT NULL,
    status INTEGER NOT NULL,
    book INTEGER,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)