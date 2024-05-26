use axum::{
    routing::{get, post},
    Router,
};
use chat::{
    database,
    router::{create_user_request, returns_json, say_hello_world, socket_hanlder},
};
use std::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let DATABASE_URL = dotenv::var("DATABASE_URL")?;
    let connection = database::db::db_init(DATABASE_URL.as_str()).await?;
    database::db::db_migration(&connection).await?;
    let service = Router::new()
        .route("/", get(|| say_hello_world()))
        .route("/ws", get(socket_hanlder))
        .route("/user/add", post(create_user_request))
        .route("/wtf", get(returns_json))
        .with_state(connection);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1984").await?;
    axum::serve(listener, service).await?;
    Ok(())
}
