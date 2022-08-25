-- Your SQL goes here

CREATE TABLE books_user_tags_link
(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    book INTEGER NOT NULL,
    tag INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book, tag)
)