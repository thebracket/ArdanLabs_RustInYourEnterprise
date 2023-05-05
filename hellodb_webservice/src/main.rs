use axum::{routing::get, Router, Json, Extension};
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let pool = sqlx::SqlitePool::connect("sqlite:hello_db.db").await.unwrap();
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Unable to migrate database");

    let app = Router::new()
        .route("/", get(say_hello_json))
        .layer(Extension(pool));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct HelloJson {
    id: i64,
    message: String,
}

async fn say_hello_json(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<HelloJson>> {
    let result = sqlx::query_as!(HelloJson, "SELECT * FROM messages")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(result)
}