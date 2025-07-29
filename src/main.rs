mod db;
mod handlers;
mod hashing;
mod jwt;
mod router;
mod test_utils;

use dotenv::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::router::router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_pool = db::connect().await;
    let app = router().with_state(db_pool);

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Listening on {}", address.to_string());
    axum::serve(listener, app).await.unwrap();
}
