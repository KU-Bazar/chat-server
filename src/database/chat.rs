use chrono::Utc;
use sqlx::{query, Error, PgPool};
use uuid::Uuid;

use crate::models::{chat::ConnectedChatUser, message::Message};

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
    receiver_id: Uuid,
    content: String,
    pool: &PgPool,
) -> Result<Message, Error> {
    let inserted_message = sqlx::query_as!(
        Message,
        r#"
        INSERT INTO Messages (chat_id, sender_id, receiver_id,content, sent_at, seen)
        VALUES ($1, $2, $3, $4, $5,$6)
        RETURNING message_id, chat_id, sender_id, receiver_id,content, sent_at, seen
        "#,
        chat_id,
        sender_id,
        receiver_id,
        content,
        Utc::now().naive_utc(),
        false
    )
    .fetch_one(pool)
    .await?;
    Ok(inserted_message)
}

pub async fn private_chat(
    sender_id: Uuid,
    receiver_id: Uuid,
    message_content: String,
    pool: &PgPool,
) -> Result<Message, Error> {
    let chat_id = connect_chat_id(sender_id, receiver_id, pool).await?;
    let saved_message = insert_message(
        chat_id,
        sender_id,
        receiver_id,
        message_content.clone(),
        pool,
    )
    .await?;
    update_chat_last_message(chat_id, message_content, sender_id, pool).await?;
    Ok(saved_message)
}

pub async fn connect_chat_id(user1_id: Uuid, user2_id: Uuid, pool: &PgPool) -> Result<i32, Error> {
    let chat_exists = check_chat_exists(user1_id, user2_id, &pool).await?;
    if !chat_exists {
        insert_chat(user1_id, user2_id, &pool).await?;
    }
    let chat_id = get_chat_id(user1_id, user2_id, &pool).await?;
    Ok(chat_id)
}

pub async fn get_all_chats(
    user1_id: Uuid,
    user2_id: Uuid,
    pool: &PgPool,
) -> Result<Vec<Message>, Error> {
    let chat_id = get_chat_id(user1_id, user2_id, pool).await?;
    let messages = sqlx::query_as!(
        Message,
        r#"
        SELECT message_id, chat_id, sender_id, receiver_id,content, sent_at, seen
        FROM Messages
        WHERE chat_id = $1
        ORDER BY sent_at DESC 
        "#,
        chat_id
    )
    .fetch_all(pool)
    .await?;
    Ok(messages)
}

pub async fn mark_messages_as_seen_for_users(
    receiver_id: Uuid,
    sender_id: Uuid,
    pool: &PgPool,
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        UPDATE Messages
        SET seen = true
        WHERE receiver_id = $2 AND sender_id = $1
        "#,
        receiver_id,
        sender_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_all_conversations(
    user: Uuid,
    pool: &PgPool,
) -> Result<Vec<ConnectedChatUser>, Error> {
    let conversations = sqlx::query_as!(
        ConnectedChatUser,
        r#"
        SELECT
            u.id,
            u.fullname,
            u.avatar_url,
            c.last_message,
            c.last_message_sent_at,
            c.last_message_sender_id,
            COUNT(m.seen) FILTER (WHERE m.seen = false AND m.receiver_id = $1) AS unseen_messages_count
        FROM
            Chats c
        JOIN
            chat_user u ON (u.id = c.user1_id OR u.id = c.user2_id)
        LEFT JOIN
            Messages m ON c.chat_id = m.chat_id
        WHERE
            (c.user1_id = $1 OR c.user2_id = $1)
            AND u.id != $1
        GROUP BY
            u.id, u.fullname, u.avatar_url, c.last_message, c.last_message_sent_at, c.last_message_sender_id
        ORDER BY
            c.last_message_sent_at ASC
    "#,
        user
    )
    .fetch_all(pool)
    .await?;
    Ok(conversations)
}
