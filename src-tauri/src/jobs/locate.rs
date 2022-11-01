use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use tauri::AppHandle;
use libminer::Client;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::pin::Pin;
use std::future::Future;

use super::Miner;
use super::JobDef;

async fn set_locate(miner: Miner, locate: bool) -> Result<()> {
    miner.set_blink(locate).await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocateJob {
    ips: Vec<String>,
    locate: bool,
}

#[async_trait]
impl JobDef for LocateJob {
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
                    Box::pin(set_locate(miner, self.locate))
                    as Pin<Box<dyn Future<Output = Result<()>> + Send>>
                );
            }
        }
        Ok(futures)
    }
}
