CREATE TABLE Chats (
    chat_id SERIAL PRIMARY KEY,
    user1_id UUID REFERENCES chat_user(id),
    user2_id UUID REFERENCES chat_user(id),
    last_message TEXT,
    last_message_sent_at TIMESTAMP,
    last_message_sender_id UUID REFERENCES chat_user(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


