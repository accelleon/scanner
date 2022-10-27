-- Add migration script here
CREATE TABLE IF NOT EXISTS pools {
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    pool1url TEXT NOT NULL,
    pool1user TEXT NOT NULL,
    pool1pass TEXT,
    pool1ipsuffix INT,
    pool2url TEXT NOT NULL,
    pool2user TEXT NOT NULL,
    pool2pass TEXT,
    pool2ipsuffix INT,
    pool3url TEXT NOT NULL,
    pool3user TEXT NOT NULL,
    pool3pass TEXT,
    pool3ipsuffix INT,
}

CREATE TABLE IF NOT EXISTS miner_auth {
    id INTEGER PRIMARY KEY NOT NULL,
    make TEXT NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
}
