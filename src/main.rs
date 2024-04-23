use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "my_api=debug".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://willie:Gn220304@localhost:5432/my_security")
        .await
    {
        Ok(pool) => pool,
        Err(e) => panic!("Couldn't establish DB connection: {}", e),
    };

    //sqlx::migrate!()
    //    .run(&db_pool)
    //    .await
    //    .expect("Cannot run migration");

    security_api::get_security_all_code(&db_pool).await.unwrap();
}
