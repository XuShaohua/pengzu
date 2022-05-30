-- Your SQL goes here
--- CREATE TABLE books (id INTEGER PRIMARY KEY AUTOINCREMENT,
---                     title     TEXT NOT NULL DEFAULT 'Unknown' COLLATE NOCASE,
---                     sort      TEXT COLLATE NOCASE,
---                     timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
---                     pubdate   TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
---                     series_index REAL NOT NULL DEFAULT 1.0,
---                     author_sort TEXT COLLATE NOCASE,
---                     isbn TEXT DEFAULT "" COLLATE NOCASE,
---                     lccn TEXT DEFAULT "" COLLATE NOCASE,
---                     path TEXT NOT NULL DEFAULT "",
---                     flags INTEGER NOT NULL DEFAULT 1,
---                     uuid TEXT,
---                     has_cover BOOL DEFAULT 0,
---                     last_modified TIMESTAMP NOT NULL DEFAULT "2000-01-01 00:00:00+00:00")

CREATE TABLE books (
	id SERIAL PRIMARY KEY,
	title TEXT NOT NULL DEFAULT 'Unknown',
	sort TEXT NOT NULL DEFAULT '',
	author_sort TEXT NOT NULL DEFAULT '',
	path TEXT NOT NULL DEFAULT '',
	uuid TEXT NOT NULL,
	has_cover BOOLEAN NOT NULL DEFAULT FALSE,
	timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	pubdate timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
