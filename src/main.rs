mod db;
mod hashing;
mod jwt;
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

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("Listening on {}", address.to_string());
    axum::serve(listener, app).await.unwrap();
}
