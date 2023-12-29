-- Add migration script here

create table notes (
    id          uuid primary key not null,
    label       text,
    contents    text not null
);
