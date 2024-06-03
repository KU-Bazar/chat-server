use socketioxide::extract::SocketRef;

pub async fn on_connect_handler(socket: SocketRef) {
    socket.on("message", |s: SocketRef| {
        s.emit("message-back", "Hello World!").ok();
    });
}
