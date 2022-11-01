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
        let row = sqlx::query!("SELECT value FROM config WHERE key = 'config'")
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
                let serial = serde_json::to_string(&default)?;
                sqlx::query!("INSERT INTO config (key, value) VALUES ('config', ?)",
                    serial
                )
                    .execute(db)
                    .await?;
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
        sqlx::query!("UPDATE config SET value = ? WHERE key = 'config'",
            serial
        )
            .execute(db)
            .await?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Pool {
    pub name: String,
    pub url1: String,
    pub url2: String,
    pub url3: String,
    pub username: String,
    pub password: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Pools {
    pub pools: Vec<Pool>,
}

impl Pools {
    pub fn new() -> Self {
        Self {
            pools: Vec::new(),
        }
    }

    pub async fn load(db: &SqlitePool) -> Result<Self> {
        let row = sqlx::query!("SELECT value FROM config WHERE key = 'pools'")
            .fetch_one(db)
            .await;
        match row {
            Err(sqlx::Error::RowNotFound) => {
                let default = Pools::new();
                sqlx::query!("INSERT INTO config (key, value) VALUES ('pools', ?)",
                    serde_json::to_string(&default)?
                )
                    .execute(db)
                    .await?;
                Ok(default)
            },
            Ok(row) => Ok(serde_json::from_str(&row.value)?),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn save(&self, db: &SqlitePool) -> Result<()> {
        let serial = serde_json::to_string(self)?;
        sqlx::query!("UPDATE config SET value = ? WHERE key = 'pools'",
            serial
        )
            .execute(db)
            .await?;
        Ok(())
    }
}
