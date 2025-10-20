use sqlx::{postgres::PgPoolOptions, PgPool};

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

    // Ensure `crates/infrastructure/migrations` exists at build time.
    // Use the macro with no path so it looks for `migrations` in the crate root.
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}
