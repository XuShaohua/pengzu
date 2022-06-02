-- Your SQL goes here

CREATE TABLE authors
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL COLLATE NOCASE,
    sort TEXT COLLATE NOCASE,
    link TEXT NOT NULL DEFAULT "",
    UNIQUE (name)
);

CREATE TABLE books
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL DEFAULT 'Unknown' COLLATE NOCASE,
    sort TEXT COLLATE NOCASE,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    pubdate TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    series_index REAL NOT NULL DEFAULT 1.0,
    author_sort TEXT COLLATE NOCASE,
    isbn TEXT DEFAULT "" COLLATE NOCASE,
    lccn TEXT DEFAULT "" COLLATE NOCASE,
    path TEXT NOT NULL DEFAULT "",
    flags INTEGER NOT NULL DEFAULT 1,
    uuid TEXT,
--     has_cover BOOLEAN NOT NULL DEFAULT 0,
    last_modified TIMESTAMP NOT NULL DEFAULT "2000-01-01 00:00:00+00:00"
);

CREATE TABLE books_authors_link
(
    id INTEGER PRIMARY KEY NOT NULL,
    book INTEGER NOT NULL,
    author INTEGER NOT NULL,
    UNIQUE (book, author)
);

CREATE TABLE books_languages_link
(
    id INTEGER PRIMARY KEY NOT NULL,
    book INTEGER NOT NULL,
    lang_code INTEGER NOT NULL,
    item_order INTEGER NOT NULL DEFAULT 0,
    UNIQUE (book, lang_code)
);

CREATE TABLE books_publishers_link
(
    id INTEGER PRIMARY KEY NOT NULL,
    book INTEGER NOT NULL,
    publisher INTEGER NOT NULL,
    UNIQUE (book)
);

CREATE TABLE books_ratings_link
(
    id INTEGER PRIMARY KEY NOT NULL,
    book INTEGER NOT NULL,
    rating INTEGER NOT NULL,
    UNIQUE (book, rating)
);

CREATE TABLE books_series_link
(
    id INTEGER PRIMARY KEY NOT NULL,
    book INTEGER NOT NULL,
    series INTEGER NOT NULL,
    UNIQUE (book)
);

CREATE TABLE books_tags_link
(
    id INTEGER PRIMARY KEY NOT NULL,
    book INTEGER NOT NULL,
    tag INTEGER NOT NULL,
    UNIQUE (book, tag)
);

CREATE TABLE comments
(
    id INTEGER PRIMARY KEY NOT NULL,
    book INTEGER NOT NULL,
    text TEXT NOT NULL COLLATE NOCASE,
    UNIQUE (book)
);

CREATE TABLE data
(
    id INTEGER PRIMARY KEY NOT NULL,
    book INTEGER NOT NULL,
    format TEXT NOT NULL COLLATE NOCASE,
    uncompressed_size INTEGER NOT NULL,
    name TEXT NOT NULL,
    UNIQUE (book, format)
);

CREATE TABLE identifiers
(
    id INTEGER PRIMARY KEY NOT NULL,
    book INTEGER NOT NULL,
    type TEXT NOT NULL DEFAULT "isbn" COLLATE NOCASE,
    val TEXT NOT NULL COLLATE NOCASE,
    UNIQUE (book, type)
);

CREATE TABLE languages
(
    id INTEGER PRIMARY KEY NOT NULL,
    lang_code TEXT NOT NULL COLLATE NOCASE,
    UNIQUE (lang_code)
);


CREATE TABLE publishers
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL COLLATE NOCASE,
    sort TEXT COLLATE NOCASE,
    UNIQUE (name)
);

CREATE TABLE ratings
(
    id INTEGER PRIMARY KEY NOT NULL,
    rating INTEGER NOT NULL CHECK (rating > -1 AND rating < 11),
    UNIQUE (rating)
);

CREATE TABLE series
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL COLLATE NOCASE,
    sort TEXT COLLATE NOCASE,
    UNIQUE (name)
);

CREATE TABLE tags
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL COLLATE NOCASE,
    UNIQUE (name)
);
