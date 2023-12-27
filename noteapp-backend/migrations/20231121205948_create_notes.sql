-- Add migration script here

create table notes (
    id          uuid primary key,
    label       text,
    contents    text not null
)
