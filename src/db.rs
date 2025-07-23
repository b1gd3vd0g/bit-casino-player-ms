use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set!");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
