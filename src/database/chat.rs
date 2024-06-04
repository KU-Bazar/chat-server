use chrono::Utc;
use sqlx::{query, Error, PgPool};
use uuid::Uuid;

pub async fn check_chat_exists(
    user1_id: Uuid,
    user2_id: Uuid,
    pool: &PgPool,
) -> Result<bool, Error> {
    let chat_exists = query!(
        r#"
        SELECT EXISTS (
            SELECT 1 FROM Chats
            WHERE (user1_id = $1 AND user2_id = $2)
            OR (user1_id = $2 AND user2_id = $1)
        ) AS "chat_exists?"
        "#,
        user1_id,
        user2_id
    )
    .fetch_one(pool)
    .await?
    .chat_exists
    .unwrap_or(false);
    Ok(chat_exists)
}

pub async fn insert_chat(user1_id: Uuid, user2_id: Uuid, pool: &PgPool) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO Chats (user1_id, user2_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4)
        "#,
        user1_id,
        user2_id,
        Utc::now().naive_utc(),
        Utc::now().naive_utc()
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_chat_last_message(
    chat_id: i32,
    content: String,
    sender_id: Uuid,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE Chats
        SET last_message = $1, last_message_sent_at = $2, last_message_sender_id = $3
        WHERE chat_id = $4
        "#,
        content,
        Utc::now().naive_utc(),
        sender_id,
        chat_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_chat_id(user1_id: Uuid, user2_id: Uuid, pool: &PgPool) -> Result<i32, Error> {
    let chat_id = sqlx::query!(
        r#"
        SELECT chat_id FROM Chats
        WHERE (user1_id = $1 AND user2_id = $2)
        OR (user1_id = $2 AND user2_id = $1)
        "#,
        user1_id,
        user2_id
    )
    .fetch_one(pool)
    .await?
    .chat_id;
    Ok(chat_id)
}

pub async fn insert_message(
    chat_id: i32,
    sender_id: Uuid,
    content: String,
    pool: &PgPool,
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO Messages (chat_id, sender_id, content, sent_at)
        VALUES ($1, $2, $3, $4)
        "#,
        chat_id,
        sender_id,
        content,
        Utc::now().naive_utc()
    )
    .execute(pool)
    .await?;
    Ok(())
}

//assuming user1 as sender_id
//assuming user2 as receiver_id
pub async fn private_chat(
    user1_id: Uuid,
    user2_id: Uuid,
    message_content: String,
    pool: &PgPool,
) -> Result<(), Error> {
    let chat_exists = check_chat_exists(user1_id, user2_id, &pool).await?;
    if !chat_exists {
        insert_chat(user1_id, user2_id, &pool).await?;
    }
    let chat_id = get_chat_id(user1_id, user2_id, &pool).await?;
    insert_message(chat_id, user1_id, message_content.clone(), &pool).await?;
    update_chat_last_message(chat_id, message_content, user1_id, &pool).await?;
    Ok(())
}

pub async fn connect_chat_id(user1_id: Uuid, user2_id: Uuid, pool: &PgPool) -> Result<i32, Error> {
    let chat_exists = check_chat_exists(user1_id, user2_id, &pool).await?;
    if !chat_exists {
        insert_chat(user1_id, user2_id, &pool).await?;
    }
    let chat_id = get_chat_id(user1_id, user2_id, &pool).await?;
    Ok(chat_id)
}
