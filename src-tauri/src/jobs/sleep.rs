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

async fn set_sleep(miner: Miner, sleep: bool) -> Result<()> {
    miner.set_sleep(sleep).await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SleepJob {
    ips: Vec<String>,
    sleep: bool,
}

#[async_trait]
impl JobDef for SleepJob {
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
                    Box::pin(set_sleep(miner, self.sleep))
                    as Pin<Box<dyn Future<Output = Result<()>> + Send>>
                );
            }
        }
        Ok(futures)
    }
}
