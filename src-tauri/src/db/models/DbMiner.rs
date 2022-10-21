use sqlx::sqlite::{SqlitePool};
use anyhow::Result;
use serde::Serialize;

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
}