use serde::Deserialize;
use serde::Serialize;
use sqlx::types::Uuid;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub fullname: String,
    pub avatar_url: Option<String>,
}
impl User {
    pub fn new(id: Uuid, username: String, fullname: String, avatar_url: Option<String>) -> User {
        User {
            id,
            username,
            fullname,
            avatar_url,
        }
    }
}
