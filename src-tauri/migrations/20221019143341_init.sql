-- Add migration script here
CREATE TABLE IF NOT EXISTS cans (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS racks (
    id INTEGER PRIMARY KEY NOT NULL,
    can_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    index_ INTEGER NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    FOREIGN KEY (can_id) REFERENCES cans(id)
    CONSTRAINT unique_rack UNIQUE (can_id, name)
);

CREATE TABLE IF NOT EXISTS miners (
    id INTEGER PRIMARY KEY NOT NULL,
    rack_id INTEGER NOT NULL,
    ip TEXT NOT NULL,
    row INTEGER NOT NULL,
    index_ INTEGER NOT NULL,
    FOREIGN KEY (rack_id) REFERENCES racks(id)
    CONSTRAINT unique_miner UNIQUE (rack_id, row, index_)
);

CREATE TABLE IF NOT EXISTS version (
    version INTEGER NOT NULL
);

INSERT INTO version (version) VALUES (1);