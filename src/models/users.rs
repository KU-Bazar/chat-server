use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub username: String,
    pub fullname: String,
}
impl User {
    pub fn new(username: String, fullname: String) -> User {
        User { username, fullname }
    }
}
