use sqlx::sqlite::{SqlitePool};
use anyhow::Result;
use serde::Serialize;

use super::rack::DbRack;

#[derive(Serialize, Debug)]
pub struct DbMiner {
    #[serde(skip)]
    pub id: i64,
    #[serde(skip)]
    pub rack_id: i64,
    pub ip: String,
    #[serde(skip)]
    pub row: i64,
    #[serde(skip)]
    pub index: i64,
}

impl DbMiner {
    pub async fn all(db: &SqlitePool) -> Result<Vec<DbMiner>> {
        let rows = sqlx::query!("SELECT id, rack_id, ip, row, index_ FROM miners")
            .fetch_all(db).await?;
        Ok(
            rows.into_iter().map(|row| {
                DbMiner {
                    id: row.id,
                    rack_id: row.rack_id,
                    ip: row.ip,
                    row: row.row,
                    index: row.index_,
                }
            }).collect()
        )
    }

    pub async fn query_rack(db: &SqlitePool, rack_id: i64) -> Result<Vec<DbMiner>> {
        let mut rows = sqlx::query!("SELECT id, rack_id, ip, row, index_ FROM miners WHERE rack_id = ? ORDER BY row, index_", rack_id)
            .fetch_all(db).await?;
        Ok(
            rows.into_iter().map(|row| {
                DbMiner {
                    id: row.id,
                    rack_id: row.rack_id,
                    ip: row.ip,
                    row: row.row,
                    index: row.index_,
                }
            }).collect()
        )
    }

    pub async fn insert(db: &SqlitePool, miner: &DbMiner) -> Result<()> {
        sqlx::query!(
            "INSERT INTO miners (rack_id, ip, row, index_) VALUES (?, ?, ?, ?)",
            miner.rack_id,
            miner.ip,
            miner.row,
            miner.index,
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn find_ip(db: &SqlitePool, ip: &str) -> Result<DbMiner> {
        let row = sqlx::query!("SELECT id, rack_id, ip, row, index_ FROM miners WHERE ip = ?", ip)
            .fetch_one(db).await?;
        Ok(
            DbMiner {
                id: row.id,
                rack_id: row.rack_id,
                ip: row.ip,
                row: row.row,
                index: row.index_,
            }
        )
    }

    pub async fn get_rack(&self, db: &SqlitePool) -> Result<DbRack> {
        DbRack::get(db, self.rack_id).await
    }
}
