#![warn(clippy::all, clippy::pedantic)]

use std::env;

use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Debug, Clone)]
pub struct Repository {
    pub connection: PgPool,
}

impl Repository {
    pub async fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let db_pool = PgPoolOptions::new()
            .connect(&database_url)
            .await.expect(&format!("Error connecting to {}", database_url));

        Repository {
            connection: db_pool,
        }
    }
}
