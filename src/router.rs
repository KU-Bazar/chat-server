use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    http::StatusCode,
    response::Response,
    Json,
};
use sqlx::PgPool;

use crate::{
    database::user::create_user,
    models::{self, users::User},
    utility::response::internal_error,
};
pub async fn say_hello_world() -> String {
    String::from("Hello world")
}

pub async fn socket_hanlder(ws: WebSocketUpgrade) -> Response {
    println!("hi i am connected!");
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
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
pub async fn create_user_request(
    State(pool): State<PgPool>,
    Json(user): Json<models::users::User>,
) -> Result<Json<User>, (StatusCode, String)> {
    // let res = create_user(user.clone(), &pool)
    //     .await
    //     .map_err(internal_error);
    let query = "insert into chat_user (username, fullname) values ($1,$2)";
    let _response = sqlx::query(query)
        .bind(&user.username)
        .bind(&user.fullname)
        .execute(&pool)
        .await
        .map_err(internal_error);
    let query = "select * from chat_user";
    Ok(Json(user))
}
