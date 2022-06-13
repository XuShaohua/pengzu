-- Your SQL goes here

CREATE TABLE books_categories_link
(
    id SERIAL PRIMARY KEY NOT NULL,
    book INTEGER NOT NULL,
    category INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(book, category)
)