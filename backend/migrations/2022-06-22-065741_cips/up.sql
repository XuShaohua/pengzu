-- Your SQL goes here

CREATE TABLE cips
(
    id SERIAL PRIMARY KEY NOT NULL,
    cip INTEGER NOT NULL,
    isbn13 INTEGER NOT NULL,
    title TEXT NOT NULL,
    original_title TEXT,
    category_id TEXT NOT NULL,
    publisher INTEGER NOT NULL,
    pubdate TEXT NOT NULL,
    price TEXT,
    intro TEXT,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(cip)
)