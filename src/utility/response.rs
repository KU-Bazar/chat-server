use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseMessage {
    pub status: String,
    pub message: String,
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub fn success_response(message: &str) -> Json<ResponseMessage> {
    let data = ResponseMessage {
        status: "Success".to_string(),
        message: message.to_string(),
    };
    return Json(data);
}

pub fn failure_response(message: &str) -> Json<ResponseMessage> {
    let data = ResponseMessage {
        status: "Failure".to_string(),
        message: message.to_string(),
    };
    return Json(data);
}
