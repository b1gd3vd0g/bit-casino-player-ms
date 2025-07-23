use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{auth, models::user::User};

#[derive(Deserialize)]
pub struct RegistrationPayload {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct RegistrationResponse {
    id: Uuid,
    username: String,
    email: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

/// Register a new user.
pub async fn register_user(
    State(pool): State<PgPool>,
    Json(payload): Json<RegistrationPayload>,
) -> Result<Json<RegistrationResponse>, (StatusCode, String)> {
    let hashed = match auth::hash_password(&payload.password) {
        Ok(str) => str,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Could not hash password."),
            ))
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
        Ok(user) => Ok(Json(RegistrationResponse {
            id: user.id,
            username: user.username,
            email: user.email,
        })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn validate_login(
    State(pool): State<PgPool>,
    Json(payload): Json<RegistrationPayload>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let authn_failed = String::from("Authentication failed!");
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
        Err(_) => return Err((StatusCode::UNAUTHORIZED, authn_failed)),
    };

    match auth::verify_password(&payload.password, &user.hashed_password) {
        Ok(_) => Ok(Json(LoginResponse {
            token: String::from("blablabla"),
        })),
        Err(_) => Err((StatusCode::UNAUTHORIZED, authn_failed)),
    }
}

/// The router handling all user paths.
pub fn user_routes() -> Router<PgPool> {
    Router::new()
        .route("/register", post(register_user))
        .route("/authn", post(validate_login))
}
