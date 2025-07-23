use axum::{routing::post, Router};
use sqlx::PgPool;

use crate::routes::{login, registration};

/// The router handling all user paths.
pub fn user_routes() -> Router<PgPool> {
    Router::new()
        .route("/register", post(registration::register_user))
        .route("/authn", post(login::validate_login))
}
