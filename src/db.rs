//! The db module contains a public `connect()` function at its root which generates a connection
//! pool to our postgres database. It also contains the module `models` with all of our database
//! models, as well as the module `queries` which contains functions to simplify making any
//! necessary database queries.

pub mod models;

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set!");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
