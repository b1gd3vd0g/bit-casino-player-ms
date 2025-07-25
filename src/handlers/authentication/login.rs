use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    db::queries::get_player_by_username,
    handlers::responses::{MessageResponse, TokenResponse},
    hashing,
    jwt::{encode_authn_token, AuthnTokenReqs},
};

/// The expected request body shape for the login request.
#[derive(Deserialize)]
pub struct ReqBody {
    username: String,
    password: String,
}

pub async fn handle_login(State(pool): State<PgPool>, Json(body): Json<ReqBody>) -> Response {
    let authn_failed = (
        StatusCode::UNAUTHORIZED,
        Json(MessageResponse::new("Authentication failed.")),
    )
        .into_response();

    let player = get_player_by_username(&pool, body.username).await;

    let player = match player {
        Ok(p) => p,
        Err(_) => return authn_failed,
    };

    let pw_match = match hashing::verify_password(&body.password, &player.password) {
        Ok(b) => b,
        Err(_) => return authn_failed,
    };

    let token_reqs = match pw_match {
        true => AuthnTokenReqs::new(player.id, player.username, player.email),
        false => return authn_failed,
    };

    match encode_authn_token(token_reqs) {
        Ok(token) => (StatusCode::OK, Json(TokenResponse::new(token))).into_response(),
        Err(_) => authn_failed,
    }
}
