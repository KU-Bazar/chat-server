use axum::{routing::get, Router};
use chat::router::{say_hello_world, socket_hanlder};
use std::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let service = Router::new()
        .route("/", get(|| say_hello_world()))
        .route("/ws", get(socket_hanlder));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1984").await?;
    axum::serve(listener, service).await?;
    Ok(())
}
