-- Add migration script here

create table users (
    id          UUID PRIMARY KEY not null,
    email       text not null,
    username    text not null,
    password    text not null
)
