use serde::Deserialize;
use serde::Serialize;
use sqlx::types::Uuid;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub fullname: String,
}
impl User {
    pub fn new(id: Uuid, username: String, fullname: String) -> User {
        User {
            id,
            username,
            fullname,
        }
    }
}
