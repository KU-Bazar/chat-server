use axum::{
    routing::{get, post},
    Router,
};
use chat::{
    database,
    router::{
        create_user_request, get_all_user_request, get_conversations_request, returns_json,
        say_hello_world, socket_handler,
    },
    socketcontroller::controller::on_connect_handler,
};
use http::Method;
use socketioxide::SocketIo;
use std::error;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let database_url = dotenv::var("DATABASE_URL")?;
    let connection = database::db::db_init(database_url.as_str()).await?;
    database::db::db_migration(&connection).await?;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    //socket
    let (socker_layer, io) = SocketIo::builder()
        .with_state(connection.clone())
        .build_layer();
    io.ns("/connect", on_connect_handler);

    //other routes
    let service = Router::new()
        .route("/", get(|| say_hello_world()))
        .route("/ws", get(socket_handler))
        .route("/user/add", post(create_user_request))
        .route("/user/getall", get(get_all_user_request))
        .route("/conversations/:id", get(get_conversations_request))
        .route("/wtf", get(returns_json))
        .with_state(connection)
        .layer(socker_layer)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1984").await?;
    axum::serve(listener, service).await?;
    Ok(())
}
