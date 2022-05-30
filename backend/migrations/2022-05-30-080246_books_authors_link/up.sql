-- Your SQL goes here
--- CREATE TABLE books_authors_link ( id INTEGER PRIMARY KEY,
---                                           book INTEGER NOT NULL,
---                                           author INTEGER NOT NULL,
---                                           UNIQUE(book, author)
---                                         )

CREATE TABLE books_authors_link (
	id SERIAL PRIMARY KEY,
	book INTEGER NOT NULL,
	author INTEGER NOT NULL,
	UNIQUE(book, author)
)
