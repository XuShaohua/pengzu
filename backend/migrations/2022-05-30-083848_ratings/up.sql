-- Your SQL goes here

-- CREATE TABLE books_ratings_link (
--     id INTEGER PRIMARY KEY,
--     book INTEGER NOT NULL,
--     rating INTEGER NOT NULL,
--     UNIQUE (book, rating)
-- )

CREATE TABLE ratings
(
    id SERIAL PRIMARY KEY,
    book INTEGER NOT NULL,
    rating INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book)
)