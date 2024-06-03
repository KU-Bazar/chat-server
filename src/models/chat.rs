use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Chat {
    pub chat_id: i32,
    pub user1_id: Uuid,
    pub user2_id: Uuid,
    pub last_message: Option<String>,
    pub last_message_sent_at: Option<chrono::NaiveDateTime>,
    pub last_message_sender_id: Option<Uuid>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
