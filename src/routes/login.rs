//! This module contains the HTTP handler for validating user login credentials, as well as any
//! related structs.
//!

use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    hashing,
    jwt::{encode_authn_token, AuthnTokenReqs},
    models::user::User,
    routes::responses::{ErrorResponse, ResponseBody, TokenResponse},
};

/// The expected request body shape for the login request.
#[derive(Deserialize)]
pub struct ReqBody {
    username: String,
    password: String,
}

/// The HTTP handler for validating a user's login credentials. Generates an authentication token
/// upon success.
///
/// # Parameters
///
/// * `pool`: The database connection pool.
/// * `payload`: The request body containing the username and password.
///
/// # Returns
///
/// A tuple containing the HTTP status code and the response body.
pub async fn validate_login(
    State(pool): State<PgPool>,
    Json(payload): Json<ReqBody>,
) -> (StatusCode, Json<ResponseBody<TokenResponse>>) {
    let authn_failed = (
        StatusCode::UNAUTHORIZED,
        Json(ResponseBody::Fail(ErrorResponse {
            message: String::from("Authentication failed."),
        })),
    );

    let user = sqlx::query_as!(
        User,
        r#"
        SELECT username, hashed_password, id, email FROM gamblers
        WHERE username ILIKE $1;
        "#,
        payload.username
    )
    .fetch_one(&pool)
    .await;

    let user = match user {
        Ok(u) => u,
        Err(_) => return authn_failed,
    };

    let reqs = match hashing::verify_password(&payload.password, &user.hashed_password) {
        Ok(_) => AuthnTokenReqs {
            id: user.id,
            username: user.username,
            email: user.email,
        },
        Err(_) => return authn_failed,
    };

    match encode_authn_token(reqs) {
        Ok(token) => {
            return (
                StatusCode::OK,
                Json(ResponseBody::Pass(TokenResponse { token: token })),
            )
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseBody::Fail(ErrorResponse::new(
                    "Token could not be encoded.",
                ))),
            )
        }
    }
}
