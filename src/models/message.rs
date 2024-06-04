use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Message {
    pub message_id: i32,
    pub chat_id: i32,
    pub sender_id: Uuid,
    pub content: String,
    pub sent_at: chrono::NaiveDateTime,
}
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct SocketMessage {
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub content: String,
}
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct SocketOnChatConnection {
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
}
