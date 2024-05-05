use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
};
pub async fn say_hello_world() -> String {
    String::from("Hello world")
}

pub async fn socket_hanlder(ws: WebSocketUpgrade) -> Response {
    println!("hi i am connected!");
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    socket.send("hello neer lets chat".into()).await.unwrap();
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}
