-- Add migration script here

CREATE TABLE notes (
    id          UUID PRIMARY KEY NOT NULL,
    label       TEXT,
    contents    TEXT NOT NULL
);
