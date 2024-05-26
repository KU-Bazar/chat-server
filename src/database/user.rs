use crate::models::users::User;
use sqlx::error::Error;
use sqlx::types::Uuid;
use sqlx::PgPool;
use sqlx::{query, query_as};
pub async fn create_user(user: User, pool: &PgPool) -> Result<User, Error> {
    let created_user = query_as!(
        User,
        "INSERT INTO chat_user (id, username, fullname) VALUES ($1, $2, $3) RETURNING id, username, fullname",
        user.id, user.username, user.fullname
    )
    .fetch_one(pool)
    .await?;
    Ok(created_user)
}
pub async fn check_user_exist(user_id: Uuid, pool: &PgPool) -> Result<bool, Error> {
    let is_there_fucking_user = query!(
        "SELECT EXISTS (SELECT 1 FROM chat_user WHERE id = $1)",
        user_id
    )
    .fetch_one(pool)
    .await?;
    //true for error propagation
    Ok(is_there_fucking_user.exists.unwrap_or(true))
}
