#![warn(clippy::all, clippy::pedantic)]

use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

pub struct Repository {
    pub connection: PgConnection,
}

impl Repository {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Repository {
            connection: PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
        }
    }
}
