use axum::{routing::post, Router};
use sqlx::PgPool;

use crate::routes::{deletion, login, registration};

/// The router handling all user paths.
pub fn user_routes() -> Router<PgPool> {
    Router::new()
        .route(
            "/",
            post(registration::register_user).delete(deletion::delete_player),
        )
        .route("/authn", post(login::validate_login))
}
