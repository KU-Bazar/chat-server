CREATE TABLE chat_user (
id uuid primary key,
username varchar(50) not null unique,
fullname varchar(50) not null
);

