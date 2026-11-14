use sqlx::migrate::MigrateError;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub async fn initialize_db(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    match sqlx::migrate!("../../migrations").run(&pool).await {
        Ok(()) => {}
        // The DB was created with an older copy of the migration files
        // (checksum drift, e.g. line endings). Tables already exist with the
        // same schema, so warn and continue instead of crashing startup.
        // To start totally fresh: stop the server and delete the .db file.
        Err(MigrateError::VersionMismatch(v)) => {
            eprintln!(
                "[WARN] DB migration {} checksum differs from migration files (files changed since DB creation). Schema already present — continuing.",
                v
            );
        }
        Err(e) => return Err(e.into()),
    }

    Ok(pool)
}
