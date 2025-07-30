use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    db::queries::create_new_player,
    handlers::responses::{MessageResponse, TokenResponse},
    hashing,
    jwt::{encode_authn_token, AuthnTokenReqs},
    requests::currency::create_bit_wallet,
};

/// The expected request body shape for the registration request.
#[derive(Deserialize)]
pub struct ReqBody {
    username: String,
    email: String,
    password: String,
}

pub async fn handle_player_creation(
    State(pool): State<PgPool>,
    Json(body): Json<ReqBody>,
) -> Response {
    let hash = match hashing::hash_password(&body.password) {
        Ok(hash) => hash,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(MessageResponse::new("Password could not be hashed.")),
            )
                .into_response();
        }
    };

    let player = create_new_player(&pool, body.username, body.email, hash).await;

    let token_reqs = match player {
        Ok(p) => AuthnTokenReqs::new(p.id, p.username, p.email),
        Err(_) => {
            return (
                StatusCode::CONFLICT,
                Json(MessageResponse::new("Username or email already exists.")),
            )
                .into_response()
        }
    };

    let token = match encode_authn_token(token_reqs) {
        Ok(tok) => tok,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(MessageResponse::token_creation_failure()),
            )
                .into_response()
        }
    };

    match create_bit_wallet(token.clone()).await {
        Ok(()) => return (StatusCode::CREATED, Json(TokenResponse::new(token))).into_response(),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(MessageResponse::new(
                    "Player created, but wallet could not be initialized. This should not happen!",
                )),
            )
                .into_response()
        }
    }
}
