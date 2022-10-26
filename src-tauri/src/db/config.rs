use serde::{Serialize, Deserialize};
use sqlx::sqlite::SqlitePool;
use anyhow::Result;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub refreshRate: u64,
    pub maxConnections: usize,
    pub connectionTimeout: u64,
    pub readTimeout: u64,
}

impl Config {
    pub fn new() -> Self {
        Self {
            refreshRate: 30,
            maxConnections: 500,
            connectionTimeout: 10,
            readTimeout: 15,
        }
    }

    pub async fn load(db: &SqlitePool) -> Result<Self> {
        let row = sqlx::query!("SELECT * FROM config")
            .fetch_one(db)
            .await;
        match row {
            Err(sqlx::Error::RowNotFound) | Err(sqlx::Error::Database(_)) => {
                let default = Config::new();
                sqlx::query!("CREATE TABLE IF NOT EXISTS config (
                    id INTEGER PRIMARY KEY NOT NULL,
                    key TEXT NOT NULL UNIQUE,
                    value TEXT NOT NULL
                );")
                    .execute(db)
                    .await?;
                default.save(db).await?;
                Ok(default)
            }
            Ok(row) => {
                Ok(serde_json::from_str(&row.value)?)
            }
            Err(e) => Err(e.into()),
        }
    }

    pub async fn save(&self, db: &SqlitePool) -> Result<()> {
        let serial = serde_json::to_string(self)?;
        sqlx::query!("UPDATE config SET value = ? WHERE key = 'json'",
            serial
        )
            .execute(db)
            .await?;
        Ok(())
    }
}
