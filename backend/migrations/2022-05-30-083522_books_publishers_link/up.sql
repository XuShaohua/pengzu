-- Your SQL goes here

-- CREATE TABLE books_publishers_link (
--     id INTEGER PRIMARY KEY,
--     book INTEGER NOT NULL,
--     publisher INTEGER NOT NULL,
--     UNIQUE (book)
-- )

CREATE TABLE books_publishers_link
(
    id SERIAL PRIMARY KEY,
    book INTEGER NOT NULL,
    publisher INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book)
)