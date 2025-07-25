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

    match encode_authn_token(token_reqs) {
        Ok(token) => (StatusCode::CREATED, Json(TokenResponse::new(token))).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse::token_creation_failure()),
        )
            .into_response(),
    }
}
