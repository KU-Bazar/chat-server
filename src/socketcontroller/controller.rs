use socketioxide::extract::{Data, SocketRef, State};
use sqlx::PgPool;

use crate::{database::chat::private_chat, models::message::SocketMessage};
pub async fn on_connect_handler(socket: SocketRef) {
    socket.on(
        "message",
        move |s: SocketRef, Data::<SocketMessage>(message), pool: State<PgPool>| {
            tokio::task::spawn(async move {
                handle_message(s, message, pool).await;
            });
        },
    );
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
