CREATE TABLE chat_user (
    username varchar(50),
    fullname varchar(50)
);

CREATE UNIQUE INDEX chatuser_idx ON chat_user (username);

