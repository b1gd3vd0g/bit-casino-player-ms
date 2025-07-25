use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::handlers::{
    authentication::{login::handle_login, token::handle_fetch_player_by_token},
    creation::handle_player_creation,
    deletion::handle_player_deletion,
};

pub fn router() -> Router<PgPool> {
    Router::new()
        .route(
            "/",
            post(handle_player_creation).delete(handle_player_deletion),
        )
        .route(
            "/authn",
            get(handle_fetch_player_by_token).post(handle_login),
        )
}
