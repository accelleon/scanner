use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use tauri::AppHandle;
use libminer::Client;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::pin::Pin;
use std::future::Future;

use super::JobDef;
use super::Miner;

async fn reboot(miner: Miner) -> Result<()> {
    miner.reboot().await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RebootJob {
    ips: Vec<String>,
}

#[async_trait]
impl JobDef for RebootJob {
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
                    Box::pin(reboot(miner))
                    as Pin<Box<dyn Future<Output = Result<()>> + Send>>
                );
            }
        }
        Ok(futures)
    }
}
