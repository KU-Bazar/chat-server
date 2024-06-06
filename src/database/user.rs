use crate::models::users::User;
use sqlx::error::Error;
use sqlx::types::Uuid;
use sqlx::PgPool;
use sqlx::{query, query_as};

pub async fn create_user(user: User, pool: &PgPool) -> Result<User, Error> {
    let created_user = query_as!(User,"INSERT INTO chat_user (id, username, fullname, avatar_url) VALUES ($1, $2, $3, $4) RETURNING id, username, fullname, avatar_url",
        user.id, user.username, user.fullname, user.avatar_url
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

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, Error> {
    let all_users = query_as!(
        User,
        "SELECT id, username, fullname, avatar_url FROM chat_user"
    )
    .fetch_all(pool)
    .await?;
    Ok(all_users)
}
