//! This module holds the handler function for the request to create a new player account, as well
//! as any related structs.

use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    hashing,
    jwt::{encode_authn_token, AuthnTokenReqs},
    models::user::User,
    routes::responses::{ErrorResponse, ResponseBody, TokenResponse},
};

/// The expected request body shape for the registration request.
#[derive(Deserialize)]
pub struct ReqBody {
    username: String,
    email: String,
    password: String,
}

/// The HTTP handler for registering a new user. Generates an authentication token upon success.
///
/// # Parameters
///
/// * `pool`: The database connection pool.
/// * `payload`: The request body containing the username, email, and password.
///
/// # Returns
///
/// A tuple containing the HTTP status code and the response body.
pub async fn register_user(
    State(pool): State<PgPool>,
    Json(payload): Json<ReqBody>,
) -> (StatusCode, Json<ResponseBody<TokenResponse>>) {
    let hashed = match hashing::hash_password(&payload.password) {
        Ok(str) => str,
        Err(_) => {
            let response = ResponseBody::Fail(ErrorResponse {
                message: String::from("Password could not be hashed."),
            });
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO gamblers (username, email, hashed_password)
        VALUES ($1, $2, $3)
        RETURNING id, username, email, hashed_password;
        "#,
        payload.username,
        payload.email,
        hashed
    )
    .fetch_one(&pool)
    .await;

    let token_reqs = match user {
        Ok(user) => AuthnTokenReqs {
            id: user.id,
            username: user.username,
            email: user.email,
        },
        Err(_) => {
            return (
                StatusCode::CONFLICT,
                Json(ResponseBody::Fail(ErrorResponse {
                    message: String::from("Email or username already exists."),
                })),
            )
        }
    };

    match encode_authn_token(token_reqs) {
        Ok(token) => {
            return (
                StatusCode::CREATED,
                Json(ResponseBody::Pass(TokenResponse { token: token })),
            )
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseBody::Fail(ErrorResponse {
                    message: String::from("Error creating authentication token."),
                })),
            )
        }
    }
}
