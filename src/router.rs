use crate::{
    database::{
        chat::get_all_conversations,
        user::{check_user_exist, create_user, get_all_users},
    },
    models::{chat::ConnectedChatUser, users::User},
    utility::response::{failure_response, success_response, ResponseMessage},
};
use axum::Json;
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Path, State,
    },
    http::StatusCode,
    response::Response,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    pub text: String,
    pub time: String,
}

pub async fn say_hello_world() -> String {
    String::from("Hello world")
}

pub async fn socket_handler(ws: WebSocketUpgrade, Json(message): Json<Message>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, message))
}

async fn handle_socket(mut socket: WebSocket, message: Message) {
    println!("Hello brother");
    println!("Received message from client: {:?}", message);
    // Loop to continue receiving messages from the WebSocket
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            println!("{:?}", msg);
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
// pub async fn create_user_request(
//     State(pool): State<PgPool>,
//     Json(user): Json<User>,
// ) -> Result<Json<User>, (StatusCode, String)> {
//     // let res = create_user(user.clone(), &pool)
//     //     .await
//     //     .map_err(internal_error);
//     let query = "insert into chat_user (username, fullname) values ($1,$2)";
//     let _response = sqlx::query(query)
//         .bind(&user.username)
//         .bind(&user.fullname)
//         .execute(&pool)
//         .await
//         .map_err(internal_error);
//     Ok(Json(user))
// }
//

pub async fn returns_json() -> Json<ResponseMessage> {
    return success_response("I am a bully");
}

pub async fn get_all_user_request(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<User>>, (StatusCode, Json<ResponseMessage>)> {
    match get_all_users(&pool).await {
        Ok(response) => return Ok(Json(response)),

        Err(_) => {
            return Err((
                StatusCode::CONFLICT,
                failure_response("User already exists"),
            ))
        }
    }
}

pub async fn create_user_request(
    State(pool): State<PgPool>,
    Json(user): Json<User>,
) -> Result<Json<ResponseMessage>, (StatusCode, Json<ResponseMessage>)> {
    let user_id = user.id.clone();

    match check_user_exist(user_id, &pool).await {
        Ok(true) => {
            return Err((
                StatusCode::CONFLICT,
                failure_response("User already exists"),
            ));
        }
        Ok(false) => match create_user(user, &pool).await {
            Ok(_created_user) => {
                return Ok(success_response("Successfully created user"));
            }
            Err(_) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    failure_response("Something went wrong while searching user"),
                ));
            }
        },
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                failure_response("Something went wrong while creating user"),
            ));
        }
    }
}

pub async fn get_conversations_request(
    Path(user_id): Path<Uuid>, // Extracts the UUID from the request path
    pool: State<PgPool>,
) -> Result<Json<Vec<ConnectedChatUser>>, (StatusCode, Json<ResponseMessage>)> {
    match get_all_conversations(user_id, &pool).await {
        Ok(conversations) => return Ok(Json(conversations)),
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                failure_response("Something went wrong while getting conversation"),
            ))
        }
    }
}
