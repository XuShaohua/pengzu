-- Your SQL goes here

CREATE TABLE categories
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
)