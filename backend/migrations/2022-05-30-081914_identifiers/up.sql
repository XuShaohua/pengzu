-- Your SQL goes here

--- CREATE TABLE identifiers  ( id     INTEGER PRIMARY KEY,
---                                     book   INTEGER NOT NULL,
---                                     type   TEXT NOT NULL DEFAULT "isbn" COLLATE NOCASE,
---                                     val    TEXT NOT NULL COLLATE NOCASE,
---                                     UNIQUE(book, type)
---         )

CREATE TABLE identifiers (
	id SERIAL PRIMARY KEY,
	book INTEGER NOT NULL,
	scheme TEXT NOT NULL DEFAULT 'isbn',
	value TEXT NOT NULL,
	timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	UNIQUE(book, scheme)
)
