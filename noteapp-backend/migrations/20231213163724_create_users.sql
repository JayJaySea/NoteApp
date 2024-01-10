-- Add migration script here

CREATE TABLE users (
    id          UUID PRIMARY KEY NOT NULL,
    email       TEXT NOT NULL UNIQUE,
    username    TEXT NOT NULL UNIQUE,
    password    TEXT NOT NULL
)

ALTER TABLE notes
ADD user_id uuid NOT NULL references users(id) ON DELETE cascade;
