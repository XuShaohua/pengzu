-- Your SQL goes here

-- CREATE TABLE languages (
--     id INTEGER PRIMARY KEY,
--     lang_code TEXT NOT NULL COLLATE NOCASE,
--     UNIQUE (lang_code)
-- )

CREATE TABLE languages
(
    id SERIAL PRIMARY KEY,
    lang_code TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (lang_code)
)
