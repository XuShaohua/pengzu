-- Your SQL goes here

CREATE TABLE douban_books_recommends_link
(
    id SERIAL PRIMARY KEY NOT NULL,
    book INTEGER NOT NULL,
    recommend_book INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book, recommend_book)
)