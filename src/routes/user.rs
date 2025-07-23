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

/// The router handling all user paths.
pub fn user_routes() -> Router<PgPool> {
    Router::new().route("/register", post(register_user))
}
