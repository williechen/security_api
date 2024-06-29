#![warn(clippy::all, clippy::pedantic)]

use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::{event, Level};

#[derive(Debug, Clone)]
pub struct Repository {
    pub connection: PgPool,
}

impl Repository {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => {
                event!(target: "security_api", Level::ERROR, "init db_pool {}", &e);
                panic!("Couldn't establish DB connection: {}", &e)
            }
        };
        Repository {
            connection: db_pool,
        }
    }
}
