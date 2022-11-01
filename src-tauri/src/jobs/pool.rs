use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use tauri::AppHandle;
use libminer::{Client};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use tauri::Manager;
use std::pin::Pin;
use std::future::Future;

use db::Pool;
use crate::db;
use super::JobDef;
use super::Miner;

async fn set_pool(miner: Miner, pool: Pool) -> Result<()> {
    miner.set_pool(pool).await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoolJob {
    ips: Vec<String>,
    pool: Pool,
}

#[async_trait]
impl JobDef for PoolJob {
    async fn prepare(
        &self,
        db: &SqlitePool,
        app: AppHandle,
        client: Client,
    ) -> Result<Vec<Pin<Box<dyn Future<Output = Result<()>> + Send>>>> {
        let mut futures = Vec::new();
        for ip in &self.ips {
            if let Ok(miner) = Miner::new(ip.clone(), db, client.clone(), app.clone()).await {
                futures.push(
                    Box::pin(set_pool(miner, self.pool.clone()))
                    as Pin<Box<dyn Future<Output = Result<()>> + Send>>
                );
            }
        }
        Ok(futures)
    }
}
