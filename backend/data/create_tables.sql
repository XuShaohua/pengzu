
-- books table
CREATE TABLE IF NOT EXISTS books
(
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL DEFAULT 'Unknown',
    path TEXT NOT NULL DEFAULT '',
    author_sort TEXT NOT NULL DEFAULT 'Unknown',
    uuid TEXT NOT NULL,
    has_cover BOOLEAN NOT NULL DEFAULT FALSE,
    pubdate TIMESTAMP,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


-- authors table.
CREATE TABLE IF NOT EXISTS authors
(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    link TEXT NOT NULL DEFAULT '',
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name)
);


-- books_authors_link table.
CREATE TABLE IF NOT EXISTS books_authors_link
(
    id SERIAL PRIMARY KEY,
    book INTEGER NOT NULL,
    author INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book, author)
);


-- comments table.
CREATE TABLE IF NOT EXISTS comments
(
    id SERIAL PRIMARY KEY,
    book INTEGER NOT NULL,
    text TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book)
);


-- files table.
CREATE TABLE IF NOT EXISTS files
(
    id SERIAL PRIMARY KEY,
    book INTEGER NOT NULL,
    format INTEGER NOT NULL,
    size INTEGER NOT NULL,
    name TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book, format)
);


-- identifiers table.
CREATE TABLE IF NOT EXISTS identifiers
(
    id SERIAL PRIMARY KEY,
    book INTEGER NOT NULL,
    scheme INTEGER NOT NULL,
    value TEXT NOT NULL,
    url TEXT,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book, scheme)
);


-- languages table.
CREATE TABLE IF NOT EXISTS languages
(
    id SERIAL PRIMARY KEY,
    lang_code TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (lang_code)
);


-- books_languages_link table.
CREATE TABLE IF NOT EXISTS books_languages_link (
    id SERIAL PRIMARY KEY,
    book INTEGER NOT NULL,
    language INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book, language)
);


-- publishers table.
CREATE TABLE IF NOT EXISTS publishers
(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name)
);


-- books_publishers_link table.
CREATE TABLE IF NOT EXISTS books_publishers_link
(
    id SERIAL PRIMARY KEY,
    book INTEGER NOT NULL,
    publisher INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book)
);


-- ratings table.
CREATE TABLE IF NOT EXISTS ratings
(
    id SERIAL PRIMARY KEY,
    book INTEGER NOT NULL,
    rating INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book)
);


-- tags table.
CREATE TABLE IF NOT EXISTS tags
(
    id SERIAL PRIMARY KEY,
    order_index INTEGER NOT NULL DEFAULT 0,
    name TEXT NOT NULL,
    parent INTEGER NOT NULL DEFAULT 0,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name)
);


-- books_tags_link table.
CREATE TABLE IF NOT EXISTS books_tags_link
(
    id SERIAL PRIMARY KEY,
    book INTEGER NOT NULL,
    tag INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book, tag)
);


-- file_formats table.
CREATE TABLE IF NOT EXISTS file_formats
(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name)
);


-- identifier_typs table.
CREATE TABLE IF NOT EXISTS identifier_types
(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    url_template TEXT NOT NULL DEFAULT '',
    description TEXT,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name)
);


-- import_libraries table.
CREATE TABLE IF NOT EXISTS import_libraries
(
    id SERIAL PRIMARY KEY,
    calibre_library_path TEXT NOT NULL,
    library_path TEXT NOT NULL,
    total INTEGER NOT NULL,
    finished BOOLEAN NOT NULL,
    options TEXT NOT NULL,
    worker_pid INTEGER,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


-- import_books table.
CREATE TABLE IF NOT EXISTS import_books
(
    id SERIAL PRIMARY KEY,
    library INTEGER NOT NULL,
    calibre_book INTEGER NOT NULL,
    ok BOOLEAN NOT NULL,
    book INTEGER,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


-- categories table.
CREATE TABLE IF NOT EXISTS categories
(
    id SERIAL PRIMARY KEY NOT NULL,
    order_index INTEGER NOT NULL,
    serial_number TEXT NOT NULL,
    name TEXT NOT NULL,
    url TEXT NOT NULL,
    description TEXT,
    parent INTEGER NOT NULL DEFAULT 0,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


-- books_categories_link table.
CREATE TABLE IF NOT EXISTS books_categories_link
(
    id SERIAL PRIMARY KEY NOT NULL,
    book INTEGER NOT NULL,
    category INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(book, category)
);


-- series table.
CREATE TABLE IF NOT EXISTS series (
    id   SERIAL PRIMARY KEY NOT NULL ,
    name TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name)
);


-- books_series_link table.
CREATE TABLE IF NOT EXISTS books_series_link
(
    id SERIAL PRIMARY KEY NOT NULL,
    book INTEGER NOT NULL,
    series INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book, series)
);


-- users table.
CREATE TABLE IF NOT EXISTS users
(
    id   SERIAL PRIMARY KEY NOT NULL ,
    name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    email TEXT NOT NULL,
    role INTEGER NOT NULL,
    salt TEXT NOT NULL,
    hash TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    UNIQUE (name),
    UNIQUE (email)
);


-- user_tags table.
CREATE TABLE IF NOT EXISTS user_tags
(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    order_index INTEGER NOT NULL DEFAULT 0,
    name TEXT NOT NULL,
    parent INTEGER NOT NULL DEFAULT 0,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name)
);


-- books_user_tags_link table.
CREATE TABLE IF NOT EXISTS books_user_tags_link
(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    book INTEGER NOT NULL,
    tag INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book, tag)
);


-- reading_history table.
CREATE TABLE IF NOT EXISTS reading_history
(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    book INTEGER NOT NULL,
    page INTEGER NOT NULL DEFAULT 0,
    percent INTEGER NOT NULL DEFAULT 0,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (book, user_id)
);


-- Index.
CREATE INDEX IF NOT EXISTS books_title_idx ON books (title);
CREATE INDEX IF NOT EXISTS books_authors_link_author_idx ON books_authors_link (author);
CREATE INDEX IF NOT EXISTS books_authors_link_book_idx ON books_authors_link (book);
CREATE INDEX IF NOT EXISTS books_categories_link_book_idx ON books_categories_link (book);
CREATE INDEX IF NOT EXISTS books_categories_link_category_idx ON books_categories_link (category);
CREATE INDEX IF NOT EXISTS books_languages_link_lang_idx ON books_languages_link (language);
CREATE INDEX IF NOT EXISTS books_languages_link_book_idx ON books_languages_link (book);
CREATE INDEX IF NOT EXISTS books_publishers_link_publisher_idx ON books_publishers_link (publisher);
CREATE INDEX IF NOT EXISTS books_publishers_link_book_idx ON books_publishers_link (book);
CREATE INDEX IF NOT EXISTS books_series_link_series_idx ON books_series_link (series);
CREATE INDEX IF NOT EXISTS books_series_link_book_idx ON books_series_link (book);
CREATE INDEX IF NOT EXISTS books_tags_link_tag_idx ON books_tags_link (tag);
CREATE INDEX IF NOT EXISTS books_tags_link_book_idx ON books_tags_link (book);
CREATE INDEX IF NOT EXISTS comments_book_idx ON comments (book);
CREATE INDEX IF NOT EXISTS categories_order_index_idx ON categories (order_index);
CREATE INDEX IF NOT EXISTS categories_serial_number_idx ON categories (serial_number);
CREATE INDEX IF NOT EXISTS categories_parent_idx ON categories (parent);
CREATE INDEX IF NOT EXISTS files_book_idx ON files (book);
CREATE INDEX IF NOT EXISTS files_format_idx ON files (format);
CREATE INDEX IF NOT EXISTS languages_lang_idx ON languages (lang_code);
CREATE INDEX IF NOT EXISTS publishers_name_idx ON publishers (name);
CREATE INDEX IF NOT EXISTS ratings_rating_idx ON ratings (rating);
CREATE INDEX IF NOT EXISTS ratings_book_idx ON ratings (book);
CREATE INDEX IF NOT EXISTS series_name_idx ON series (name);
CREATE INDEX IF NOT EXISTS tags_name_idx ON tags (name);
