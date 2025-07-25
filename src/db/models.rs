//! Contains all database models for the player microservice.
//!
//! For now, this only contains the `Player` model (found in the `players` table).
//!
//! Possible future enhancements include:
//!
//! - Adding a `LoginAttempt` model to track login attempts.

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

/// The Player model represents a row from the `players` table in our database.
#[derive(FromRow)]
pub struct Player {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}
