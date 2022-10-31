use sqlx::sqlite::SqlitePool;
use anyhow::Result;
use serde::Serialize;

use super::rack::DbRack;

#[derive(Serialize, Debug)]
pub struct DbCan {
    pub id: i64,
    pub name: String,
    pub num: i64,
    pub racks: Vec<DbRack>,
}

impl DbCan {
    pub async fn all(db: &SqlitePool) -> Result<Vec<DbCan>> {
        let rows = sqlx::query!("SELECT id, num, name FROM cans")
            .fetch_all(db).await?;
        let cans = rows.into_iter().map(|row| {
            DbCan {
                id: row.id,
                name: row.name,
                num: row.num,
                racks: vec![],
            }
        }).collect();
        Ok(cans)
    }

    pub async fn load_racks(&mut self, db: &SqlitePool) -> Result<()> {
        let racks = DbRack::query_can(db, self.id).await?;
        self.racks = racks;
        Ok(())
    }

    pub async fn query_name(db: &SqlitePool, name: &str) -> Result<DbCan> {
        let row = sqlx::query!("SELECT id, num, name FROM cans WHERE name = ?", name)
            .fetch_one(db).await?;
        let mut can = DbCan {
            id: row.id,
            name: row.name,
            num: row.num,
            racks: vec![],
        };
        can.load_racks(db).await?;
        Ok(can)
    }

    pub async fn get(db: &SqlitePool, id: i64) -> Result<DbCan> {
        let row = sqlx::query!("SELECT id, num, name FROM cans WHERE id = ?", id)
            .fetch_one(db).await?;
        let mut can = DbCan {
            id: row.id,
            name: row.name,
            num: row.num,
            racks: vec![],
        };
        can.load_racks(db).await?;
        Ok(can)
    }
}
