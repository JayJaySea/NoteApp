-- Add migration script here

create table notes (
    id          UUID PRIMARY KEY,
    label       text,
    contents    text not null
)
