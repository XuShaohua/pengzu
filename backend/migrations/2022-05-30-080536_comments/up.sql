-- Your SQL goes here

--- CREATE TABLE comments ( id INTEGER PRIMARY KEY,
---                               book INTEGER NOT NULL,
---                               text TEXT NOT NULL COLLATE NOCASE,
---                               UNIQUE(book)
---                             )

CREATE TABLE comments (
	id SERIAL PRIMARY KEY,
	book INTEGER NOT NULL,
	text TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	UNIQUE(book)
)
