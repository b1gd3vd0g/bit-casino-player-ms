//! Contains functions simplifying database queries.
//!
//! # Notes
//! * Some of these functions return Player structs - these contain **sensitive player data** (such
//!   as hashed passwords) and therefore should NEVER be returned to the client as-is.

use sqlx::PgPool;

use crate::{db::models::Player, jwt::AuthnTokenPayload};

/// Search for a single player by their username. This search is **case insensitive**, but it must
/// otherwise be an exact match.
///
/// # Notes
/// * The return value of this function contains the hashed password and should **never** be
///   returned to the client.
///
/// # Arguments
/// * pool - The postgres connection pool
/// * username - The username to be searched for.
///
/// # Returns
/// A Player if it can be found, and an error if not.
pub async fn get_player_by_username(
    pool: &PgPool,
    username: String,
) -> Result<Player, sqlx::Error> {
    sqlx::query_as!(
        Player,
        r#"
        SELECT * from players
        WHERE username ILIKE $1
        "#,
        username
    )
    .fetch_one(pool)
    .await
}

/// Search for a single player based on their authentication token payload.
///
/// # Notes
/// * This function does **not** decode the authentication token!
/// * The return value of this function contains the hashed password and should **never** be
///   returned to the client.
///
/// # Arguments
/// * pool - The postgres connection pool.
/// * payload - The decoded authentication token's payload.
///
/// # Returns
/// A Player if it can be found, and an error if not.
pub async fn get_player_by_token(
    pool: &PgPool,
    payload: AuthnTokenPayload,
) -> Result<Player, sqlx::Error> {
    sqlx::query_as!(
        Player,
        r#"
        SELECT * from players
        WHERE id = $1 AND username = $2 AND email = $3
        "#,
        payload.sub,
        payload.username,
        payload.email
    )
    .fetch_one(pool)
    .await
}

/// Attempt to create a new player in the database.
///
/// # Notes
/// * This function does **not** hash the password internally! Do **not** pass in an unhashed
///   password, as it will be inserted directly into the database.
/// * The **most likely** cause of failure for this function is that the username and/or email
///   **already exist** in the database.
///
/// # Arguments
/// * pool: The postgres connection pool.
/// * username: The username of the new player.
/// * email: The email address of the new player.
/// * hash: The hashed password of the new player.
pub async fn create_new_player(
    pool: &PgPool,
    username: String,
    email: String,
    hash: String,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO players (username, email, password)
        VALUES ($1, $2, $3)
        "#,
        username,
        email,
        hash
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete a single player account based on their authentication token payload.
///
/// # Notes
/// * This function does **not** decode the authentication token.
///
/// # Arguments
/// * pool - The postgres connection pool.
/// * payload - The decoded authentication token's payload.
pub async fn delete_player_by_token(
    pool: &PgPool,
    payload: AuthnTokenPayload,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM players
        WHERE id = $1 AND username = $2 AND email = $3
        "#,
        payload.sub,
        payload.username,
        payload.email
    )
    .execute(pool)
    .await?;
    Ok(())
}
