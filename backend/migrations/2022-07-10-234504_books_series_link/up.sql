-- Your SQL goes here

CREATE TABLE books_series_link
(
    id SERIAL PRIMARY KEY NOT NULL,
    book INTEGER NOT NULL,
    series INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book, series)
)