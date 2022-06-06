-- Your SQL goes here

-- CREATE TABLE books_plugin_data
-- (
--     id INTEGER PRIMARY KEY,
--     book INTEGER NOT NULL,
--     name TEXT NOT NULL,
--     val TEXT NOT NULL,
--     UNIQUE (book, name)
-- );

CREATE TABLE books_plugin_data
(
    id INTEGER PRIMARY KEY NOT NULL,
    book INTEGER NOT NULL,
    name TEXT NOT NULL,
    val TEXT NOT NULL,
    UNIQUE (book, name)
)