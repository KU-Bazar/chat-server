use socketioxide::extract::{Data, SocketRef};

use crate::models::message::SocketMessage;
pub async fn on_connect_handler(socket: SocketRef) {
    socket.on(
        "message",
        move |s: SocketRef, Data::<SocketMessage>(message)| {
            tokio::task::spawn(async move {
                handle_message(s, message).await;
            });
        },
    );
}

async fn handle_message(socket: SocketRef, message: SocketMessage) {
    println!("{:?}", message);
}
