use socketioxide::extract::{Data, SocketRef, State};
use sqlx::PgPool;

use crate::{
    database::chat::{connect_chat_id, private_chat},
    models::message::{SocketMessage, SocketOnChatConnection},
};
pub async fn on_connect_handler(socket: SocketRef, pool: State<PgPool>) {
    socket.on(
        "join_chat",
        move |s: SocketRef, Data::<SocketOnChatConnection>(message), pool: State<PgPool>| {
            tokio::task::spawn(async move {
                handle_chat_join(s, message, pool).await;
            });
        },
    );

    socket.on(
        "message",
        move |s: SocketRef, Data::<SocketMessage>(message), pool: State<PgPool>| {
            tokio::task::spawn(async move {
                handle_message(s, message, pool).await;
            });
        },
    );
}
async fn handle_chat_join(
    socket: SocketRef,
    chat_connection: SocketOnChatConnection,
    pool: State<PgPool>,
) {
    match connect_chat_id(
        chat_connection.sender_id,
        chat_connection.receiver_id,
        &pool,
    )
    .await
    {
        Ok(id) => {
            if let Err(err) = socket.leave_all() {
                eprintln!("Failed to leave all rooms: {}", err);
            }
            if let Err(err) = socket.join(id.to_string()) {
                eprintln!("Failed to join chat room: {}", err);
            }
        }
        Err(err) => {
            if let Err(err) = socket.emit("message-out", "this ain't good!") {
                eprintln!("Failed to emit message: {}", err);
            }
        }
    }
}

async fn handle_message(socket: SocketRef, message: SocketMessage, pool: State<PgPool>) {
    let chat_status = private_chat(
        message.sender_id,
        message.receiver_id,
        message.content,
        &pool,
    )
    .await;
    match chat_status {
        Ok(_) => {
            if let Err(err) = socket.emit("message-out", "Good shit happening!") {
                eprintln!("Failed to emit message: {}", err);
            }
        }
        Err(err) => {
            eprintln!("Error handling message: {}", err);
            if let Err(err) = socket.emit("message-out", "An error occurred {err}") {
                eprintln!("Failed to emit message: {}", err);
            }
        }
    };
}
