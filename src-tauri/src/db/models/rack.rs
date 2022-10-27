use serde::Serialize;
use sqlx::sqlite::SqlitePool;
use anyhow::Result;
use tokio::{spawn, join};

use super::miner::DbMiner;

#[derive(Serialize, Debug)]
pub struct DbRack {
    #[serde(skip)]
    pub id: i64,
    #[serde(skip)]
    pub can_id: i64,
    pub name: String,
    pub index: i64,
    pub width: i64,
    pub height: i64,

    pub miners: Vec<Vec<DbMiner>>,
}

impl DbRack {
    pub async fn all(db: &SqlitePool) -> Result<Vec<DbRack>> {
        let rows = sqlx::query!("SELECT id, can_id, index_, name, width, height FROM racks")
            .fetch_all(db).await?;
        let mut racks: Vec<DbRack> = rows.into_iter().map(|row| {
            DbRack {
                id: row.id,
                can_id: row.can_id,
                name: row.name,
                index: row.index_,
                width: row.width,
                height: row.height,
                miners: vec![],
            }
        }).collect();
        Ok(racks)
    }

    pub async fn query_can(db: &SqlitePool, can_id: i64) -> Result<Vec<DbRack>> {
        let rows = sqlx::query!("SELECT id, can_id, name, index_, width, height FROM racks WHERE can_id = ? ORDER BY index_", can_id)
            .fetch_all(db).await?;
        let mut racks: Vec<DbRack> = rows.into_iter().map(|row| {
            DbRack {
                id: row.id,
                can_id: row.can_id,
                name: row.name,
                index: row.index_,
                width: row.width,
                height: row.height,
                miners: vec![],
            }
        }).collect();
        // TODO: We can do this better
        for rack in &mut racks {
            rack.load_miners(db).await?;
        }
        Ok(racks)
    }

    pub async fn load_miners(&mut self, db: &SqlitePool) -> Result<()> {
        let miners = DbMiner::query_rack(db, self.id).await?;
        let mut rack_miners: Vec<Vec<DbMiner>> = vec![];
        for _ in 0..self.height {
            rack_miners.push(vec![]);
        }
        for miner in miners {
            rack_miners[miner.row as usize].push(miner);
        }
        self.miners = rack_miners;
        Ok(())
    }

    pub async fn get(db: &SqlitePool, id: i64) -> Result<DbRack> {
        let row = sqlx::query!("SELECT id, can_id, name, index_, width, height FROM racks WHERE id = ?", id)
            .fetch_one(db).await?;
        let mut rack = DbRack {
            id: row.id,
            can_id: row.can_id,
            name: row.name,
            index: row.index_,
            width: row.width,
            height: row.height,
            miners: vec![],
        };
        rack.load_miners(db).await?;
        Ok(rack)
    }
}
