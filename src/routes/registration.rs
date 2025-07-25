//! This module holds the handler function for the request to create a new player account, as well
//! as any related structs.

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    hashing,
    models::user::User,
    routes::responses::{ErrorResponse, ResponseBody},
};

/// The expected request body shape for the registration request.
#[derive(Deserialize)]
struct ReqBody {
    username: String,
    email: String,
    password: String,
}

/// The response returned after a successful registration.
/// It contains public information about the new player.
#[derive(Serialize)]
pub struct SuccessBody {
    id: Uuid,
    username: String,
    email: String,
}

/// The HTTP handler for registering a new user.
///
/// # Returns
///
/// A tuple containing the HTTP status code and the response body.
pub async fn register_user(
    State(pool): State<PgPool>,
    Json(payload): Json<ReqBody>,
) -> (StatusCode, ResponseBody<SuccessBody>) {
    let hashed = match hashing::hash_password(&payload.password) {
        Ok(str) => str,
        Err(_) => {
            let response = ResponseBody::Fail(ErrorResponse {
                message: String::from("Password could not be hashed."),
            });
            return (StatusCode::INTERNAL_SERVER_ERROR, response);
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

    match user {
        Ok(user) => {
            return (
                StatusCode::CREATED,
                ResponseBody::Pass(SuccessBody {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                }),
            )
        }
        Err(_) => {
            return (
                StatusCode::CONFLICT,
                ResponseBody::Fail(ErrorResponse {
                    message: String::from("Email or username already exists."),
                }),
            )
        }
    }
}
