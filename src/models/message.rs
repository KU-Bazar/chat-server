use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Message {
    pub message_id: i32,
    pub chat_id: Option<i32>,
    pub sender_id: Option<Uuid>,
    pub receiver_id: Option<Uuid>,
    pub content: String,
    pub sent_at: Option<chrono::NaiveDateTime>,
    pub seen: Option<bool>,
}
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SocketMessage {
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub content: String,
}
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SocketOnChatConnection {
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
}
