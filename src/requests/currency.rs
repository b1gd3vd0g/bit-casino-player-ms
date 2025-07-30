use axum::http::HeaderMap;
use reqwest::{Client, StatusCode};

use crate::handlers::responses::MessageResponse;

pub async fn create_bit_wallet(token: String) -> Result<(), MessageResponse> {
    let client = Client::new();

    let mut hm = HeaderMap::new();
    let hv = format!("Bearer {}", token);
    hm.insert("Authorization", hv.parse().unwrap());

    let response = client
        .post("http://currency-ms:3000")
        .headers(hm)
        .send()
        .await;

    let response = match response {
        Ok(r) => r,
        Err(_) => return Err(MessageResponse::new("Failed to call the currency api.")),
    };

    match response.status() {
        StatusCode::CREATED => Ok(()),
        _ => Err(MessageResponse::new(
            "The request to create a new wallet failed.",
        )),
    }
}
