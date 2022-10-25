-- Add migration script here
CREATE TABLE IF NOT EXISTS config (
    id INTEGER PRIMARY KEY NOT NULL,
    key TEXT NOT NULL UNIQUE,
    value TEXT NOT NULL
);
