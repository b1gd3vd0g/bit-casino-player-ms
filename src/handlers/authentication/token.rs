use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db::queries::get_player_by_token,
    handlers::{helper::extract_authn_token, responses::MessageResponse},
    jwt::decode_authn_token,
};

#[derive(Serialize)]
pub struct SafePlayerInfo {
    id: Uuid,
    username: String,
    email: String,
    created_at: DateTime<Utc>,
}

pub async fn handle_fetch_player_by_token(
    State(pool): State<PgPool>,
    headers: HeaderMap,
) -> Response {
    let authn_token = match extract_authn_token(headers) {
        Ok(token) => token,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(e)).into_response(),
    };

    let payload = match decode_authn_token(authn_token) {
        Ok(p) => p.claims,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(MessageResponse::token_auth_failure()),
            )
                .into_response()
        }
    };

    let player = get_player_by_token(&pool, payload).await;

    match player {
        Ok(p) => (
            StatusCode::OK,
            Json(SafePlayerInfo {
                id: p.id,
                username: p.username,
                email: p.email,
                created_at: p.created_at,
            }),
        )
            .into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(MessageResponse::new("Player could not be found.")),
        )
            .into_response(),
    }
}
