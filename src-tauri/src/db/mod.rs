use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::sqlite::SqlitePool;

pub mod models;
pub use models::DbCan::DbCan;
pub use models::DbMiner::DbMiner;
pub use models::DbRack::DbRack;

pub async fn connect() -> Result<SqlitePool, sqlx::Error> {
    let opts = SqliteConnectOptions::new()
        .filename("./scanner.db")
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await?;

    let row = sqlx::query!("SELECT version FROM version")
        .fetch_one(&pool).await;
    if row.is_err() {
        create_tables(&pool).await?;
    }
    Ok(pool)
}

pub async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS cans (
            id INTEGER PRIMARY KEY NOT NULL,
            name TEXT NOT NULL UNIQUE
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS racks (
            id INTEGER PRIMARY KEY NOT NULL,
            can_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            index_ INTEGER NOT NULL,
            width INTEGER NOT NULL,
            height INTEGER NOT NULL,
            FOREIGN KEY (can_id) REFERENCES cans(id)
            CONSTRAINT unique_rack UNIQUE (can_id, name)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS miners (
            id INTEGER PRIMARY KEY NOT NULL,
            rack_id INTEGER NOT NULL,
            ip TEXT NOT NULL,
            row INTEGER NOT NULL,
            index_ INTEGER NOT NULL,
            FOREIGN KEY (rack_id) REFERENCES racks(id)
            CONSTRAINT unique_miner UNIQUE (rack_id, row, index_)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS version (
            version INTEGER NOT NULL
        )"#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO version (version) VALUES (1)
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}