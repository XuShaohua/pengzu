-- Your SQL goes here

CREATE TABLE douban_books_detail
(
    id SERIAL PRIMARY KEY NOT NULL,
    book_id INTEGER NOT NULL,
    original_title TEXT,
    large_cover TEXT NOT NULL,
    toc TEXT,
    intro TEXT,
    publisher INTEGER,
    rating_number float4,
    rating_people INTEGER,
    price TEXT,
    page INTEGER,
    pubdate TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	UNIQUE (book_id)
)
