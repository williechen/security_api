#![warn(clippy::all, clippy::pedantic)]

use std::env;

use diesel::{r2d2, PgConnection};
use dotenvy::dotenv;

#[derive(Debug, Clone)]
pub struct Repository {
    pub connection: r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
}

impl Repository {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);

        Repository {
            connection: r2d2::Pool::builder()
                .max_size(5)
                .build(manager)
                .expect("Failed to create pool."),
        }
    }
}
