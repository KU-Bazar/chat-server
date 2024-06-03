CREATE TABLE Messages (
    message_id SERIAL PRIMARY KEY,
    chat_id INT REFERENCES Chats(chat_id),
    sender_id UUID REFERENCES chat_user(id),
    content TEXT NOT NULL,
    sent_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

