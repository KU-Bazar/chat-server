use axum::response::Result;
use socketioxide::extract::{Data, SocketRef, State};
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::{
    database::chat::{
        connect_chat_id, get_all_chats, mark_messages_as_seen_for_users, private_chat,
    },
    models::message::{Message, SocketMessage, SocketOnChatConnection},
};

pub async fn on_connect_handler(socket: SocketRef) {
    // println!("user connected: {:?}", socket.id);

    //generates chat id if not availabel and returns the chatid
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
                handle_message(s, message.clone(), pool).await;
            });
        },
    );
}

async fn handle_chat_join(
    socket: SocketRef,
    chat_connection: SocketOnChatConnection,
    pool: State<PgPool>,
) {
    println!(
        "user1 :{:?} user2: {:?} connected",
        chat_connection.sender_id, chat_connection.receiver_id
    );
    // at first the user gest connected to the chat room
    match connect_chat_id(
        chat_connection.sender_id,
        chat_connection.receiver_id,
        &pool,
    )
    .await
    {
        Ok(id) => {
            if let Err(err) = socket.leave_all() {
                return eprintln!("Failed to leave all rooms: {}", err);
            }
            if let Err(err) = socket.join(id.to_string()) {
                return eprintln!("Failed to join chat room: {}", err);
            }
            // and then the users gets the chat history
            handle_messages_seen(
                chat_connection.sender_id,
                chat_connection.receiver_id,
                &pool,
            )
            .await;
            handle_messages_history(socket, chat_connection, &pool).await;
        }
        Err(_) => {
            if let Err(err) = socket.emit("message-out", "this ain't good!") {
                eprintln!("Failed to emit message: {}", err);
            }
        }
    }
}

async fn handle_messages_seen(sender_id: Uuid, receiver_id: Uuid, pool: &PgPool) {
    match mark_messages_as_seen_for_users(receiver_id, sender_id, pool).await {
        Ok(_) => {
            println!("Messages seen by {:?} of {:?}", sender_id, receiver_id);
        }
        Err(e) => {
            eprintln!("Failed to mark messages as seen: {}", e);
        }
    }
}

async fn handle_messages_history(
    socket: SocketRef,
    chat_connection: SocketOnChatConnection,
    pool: &PgPool,
) {
    let messages_result = get_all_chats(
        chat_connection.sender_id,
        chat_connection.receiver_id,
        &pool,
    )
    .await;

    match messages_result {
        Ok(messages) => {
            if let Err(err) = socket.emit("chat-history", [messages]) {
                eprintln!("Failed to emit chat history: {}", err);
            }
        }
        Err(err) => {
            eprintln!("Failed to get chat history: {}", err);
            if let Err(err) = socket.emit("message-out", "Failed to retrieve chat history") {
                eprintln!("Failed to emit error message: {}", err);
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

    match connect_chat_id(message.sender_id, message.receiver_id, &pool).await {
        Ok(id) => {
            message_emitter(chat_status, id, socket).await;
            handle_messages_seen(message.sender_id, message.receiver_id, &pool).await;
        }
        Err(_) => {
            if let Err(err) = socket.emit("message-out", "this ain't good!") {
                eprintln!("Failed to emit message: {}", err);
            }
        }
    }
}

// emits message to only and only two users connected to the room id
async fn message_emitter(chat_status: Result<Message, Error>, id: i32, socket: SocketRef) {
    match chat_status {
        Ok(message) => {
            if let Err(err) = socket.within(id.to_string()).emit("private-chat", message) {
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
