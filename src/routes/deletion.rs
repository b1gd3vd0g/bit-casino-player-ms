use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use sqlx::PgPool;

use crate::{
    jwt::decode_authn_token,
    routes::responses::{ErrorResponse, ResponseBody},
};

pub async fn delete_player(State(pool): State<PgPool>, headers: HeaderMap) -> Response {
    let auth_header = match headers.get("Authorization") {
        Some(value) => value,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ResponseBody::<()>::Fail(ErrorResponse::token_auth_failure())),
            )
                .into_response();
        }
    };
    let auth_header = match auth_header.to_str() {
        Ok(value) => value,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ResponseBody::<()>::Fail(ErrorResponse::token_auth_failure())),
            )
                .into_response();
        }
    };
    let token = match auth_header.strip_prefix("Bearer ") {
        Some(token) => token,
        None => {
            return (
                StatusCode::FORBIDDEN,
                Json(ResponseBody::<()>::Fail(ErrorResponse::token_auth_failure())),
            )
                .into_response();
        }
    };
    let payload = match decode_authn_token(String::from(token)) {
        Ok(payload) => payload.claims,
        Err(_) => {
            return (
                StatusCode::TOO_MANY_REQUESTS,
                Json(ResponseBody::<()>::Fail(ErrorResponse::token_auth_failure())),
            )
                .into_response();
        }
    };
    let user_id = payload.sub;
    let username = payload.username;
    let email = payload.email;
    let result = sqlx::query!(
        r#"
        DELETE FROM gamblers
        WHERE id = $1 AND username = $2 AND email = $3
        "#,
        user_id,
        username,
        email
    )
    .execute(&pool)
    .await;
    match result {
        Ok(_) => return StatusCode::NO_CONTENT.into_response(),
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ResponseBody::<()>::Fail(ErrorResponse::new(
                    "User represented by the authentication token could not be found.",
                ))),
            )
                .into_response()
        }
    }
}
