-- Add migration script here
ALTER TABLE cans
ADD num INTEGER NOT NULL DEFAULT 0;
