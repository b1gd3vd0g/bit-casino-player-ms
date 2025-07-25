mod db;
mod handlers;
mod hashing;
mod jwt;
mod router;

use dotenv::dotenv;
use std::net::SocketAddr;

use crate::router::router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_pool = db::connect().await;
    let app = router().with_state(db_pool);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("Listening on {}", address.to_string());
    axum::serve(listener, app).await.unwrap();
}
