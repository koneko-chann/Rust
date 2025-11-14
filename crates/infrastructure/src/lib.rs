use sqlx::{PgPool, postgres::PgPoolOptions};
pub mod middleware;
pub mod utils;
pub async fn initialize_db(dsn: &str, max_conn: u32) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(dsn)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to connect to the database: {}", e);
            std::process::exit(1);
        });

    println!("Database connection established");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}
