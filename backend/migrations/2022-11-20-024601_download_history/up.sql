-- Your SQL goes here

CREATE TABLE download_history
(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    book INTEGER NOT NULL,
    format INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)