use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub async fn initialize_db(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}
