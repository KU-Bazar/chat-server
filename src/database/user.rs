use crate::models::users::User;
use sqlx::PgPool;

pub async fn create_user(user: User, pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let query = "INSERT INTO chat_user (username, fullname) VALUES ($1,$2)";
    sqlx::query(query)
        .bind(&user.username)
        .bind(&user.fullname)
        .execute(pool)
        .await?;
    Ok(())
}
