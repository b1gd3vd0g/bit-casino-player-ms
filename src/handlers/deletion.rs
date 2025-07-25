use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use sqlx::PgPool;

use crate::{
    db::queries::delete_player_by_token,
    handlers::{helper::extract_authn_token, responses::MessageResponse},
    jwt::decode_authn_token,
};

pub async fn handle_player_deletion(State(pool): State<PgPool>, headers: HeaderMap) -> Response {
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

    let deletion = delete_player_by_token(&pool, payload).await;

    match deletion {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(MessageResponse::new("Player could not be found.")),
        )
            .into_response(),
    }
}
