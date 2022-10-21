-- Add migration script here
CREATE TABLE IF NOT EXISTS errors (
    id SERIAL PRIMARY KEY,
    make TEXT NOT NULL,
    regex TEXT NOT NULL,
    message TEXT NOT NULL,
);

BEGIN TRANSACTION
INSERT INTO errors (regex, message) VALUES ('^.*$', 'Unknown error');