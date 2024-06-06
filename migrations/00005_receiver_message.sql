ALTER TABLE Messages
ADD COLUMN receiver_id UUID REFERENCES chat_user(id);

