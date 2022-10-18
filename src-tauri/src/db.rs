use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::sqlite::SqlitePool;

pub async fn connect() -> Result<SqlitePool, sqlx::Error> {
    let opts = SqliteConnectOptions::new()
        .filename("scanner.db")
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await?;
    Ok(pool)
}