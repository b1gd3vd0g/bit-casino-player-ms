mod auth;
mod db;
mod models;
mod routes;

use axum::Router;
use dotenv::dotenv;
use routes::user::user_routes;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_pool = db::connect().await.expect("Failed to connect to db.");
    let app = Router::new()
        .nest("/users", user_routes())
        .with_state(db_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
