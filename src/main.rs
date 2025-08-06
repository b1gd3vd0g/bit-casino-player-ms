mod db;
mod handlers;
mod hashing;
mod jwt;
mod requests;
mod router;
mod test_utils;
mod validators;

use std::{env, net::SocketAddr};

use dotenv::dotenv;
use tokio::net::TcpListener;

use crate::router::router;

#[tokio::main]
async fn main() {
    match env::var("STAGE") {
        Err(_) => {
            // local test (cargo run).
            dotenv().ok();
        }
        Ok(stage) => {
            // container test (docker run)
            if stage == "docker" {
                dotenv::from_filename(".env.docker").ok();
            }
        } // Otherwise, the env should be set elsewhere (such as in the docker-compose.yaml file)
    }

    let db_pool = db::connect().await;
    let app = router().with_state(db_pool);

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Listening on {}", address.to_string());
    axum::serve(listener, app).await.unwrap();
}
