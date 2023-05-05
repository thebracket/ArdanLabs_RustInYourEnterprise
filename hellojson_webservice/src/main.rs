use axum::{routing::get, Router, Json};
use serde::Serialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(say_hello_json));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct HelloJson {
    message: String,
}

async fn say_hello_json() -> Json<HelloJson> {
    Json(HelloJson {
        message: "Hello, World!".to_string(),
    })
}