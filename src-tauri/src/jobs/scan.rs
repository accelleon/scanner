use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use tauri::AppHandle;
use libminer::Client;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use tauri::Manager;
use std::pin::Pin;
use std::future::Future;

use crate::db;
use super::JobDef;
use super::Miner;

async fn scan(miner: Miner) -> Result<()> {
    miner.scan().await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScanJob {
    pub can: i64,
}

#[async_trait]
impl JobDef for ScanJob {
    async fn prepare(
        &self,
        db: &SqlitePool,
        app: AppHandle,
        client: Client,
    ) -> Result<Vec<Pin<Box<dyn Future<Output = Result<()>> + Send>>>> {
        let mut can = db::DbCan::get(db, self.can).await?;
        can.load_racks(db).await?;
        let auths = db::MinerAuth::load(db).await?;
        let mut futures = vec![];
        for rack in &can.racks {
            for row in &rack.miners {
                for miner in row {
                    let miner = Miner::default(
                        miner.ip.clone(),
                        rack.index, miner.row, miner.index, can.num,
                        app.clone(), client.clone(), auths.clone(),
                    );
                    futures.push(
                        Box::pin(scan(miner))
                        as Pin<Box<dyn Future<Output = Result<()>> + Send>>
                    );
                }
            }
        }
        Ok(futures)
    }
}
