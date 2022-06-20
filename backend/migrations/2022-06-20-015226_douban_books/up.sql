-- Your SQL goes here

CREATE TABLE douban_books
(
    id SERIAL PRIMARY KEY NOT NULL,
    subject_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    original_title TEXT,
    isbn TEXT NOT NULL,
    url TEXT NOT NULL,
    large_cover TEXT NOT NULL,
    small_cover TEXT NOT NULL,
    toc TEXT,
    intro TEXT,
    publisher INTEGER,
    rating_number float4,
    rating_people INTEGER,
    price TEXT,
    page INTEGER,
    pubdate TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)