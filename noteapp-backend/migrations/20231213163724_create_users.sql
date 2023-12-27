-- Add migration script here

create table users (
    id          uuid primary key not null,
    email       text not null unique,
    username    text not null unique,
    password    text not null
)

alter table notes
add user_id uuid not null references users(id) on delete cascade;
