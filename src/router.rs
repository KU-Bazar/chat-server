use crate::{
    database::user::{check_user_exist, create_user, get_all_users},
    models::users::User,
    utility::response::{failure_response, success_response, ResponseMessage},
};
use axum::Json;
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    http::StatusCode,
    response::Response,
};
use sqlx::PgPool;
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
