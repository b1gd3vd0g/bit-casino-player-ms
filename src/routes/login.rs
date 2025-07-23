use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    hashing,
    jwt::{encode_authn_token, AuthnTokenReqs},
    models::user::User,
};

#[derive(Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

pub async fn validate_login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginPayload>,
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

    let reqs = match hashing::verify_password(&payload.password, &user.hashed_password) {
        Ok(_) => AuthnTokenReqs {
            id: user.id,
            username: user.username,
            email: user.email,
        },
        Err(_) => return Err((StatusCode::UNAUTHORIZED, authn_failed)),
    };

    let token = encode_authn_token(reqs);
    match token {
        Ok(tok) => Ok(Json(LoginResponse { token: tok })),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Could not create authentication token."),
        )),
    }
}
