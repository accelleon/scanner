use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use tauri::AppHandle;
use libminer::Client;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use tauri::Manager;
use std::pin::Pin;
use std::future::Future;

use crate::models::{Miner, MinerEvent};
use crate::db;
use super::JobDef;

async fn set_sleep(client: Client, ip: &str, sleep: bool) -> Result<()> {
    if let Ok(mut miner) = client.get_miner(ip, None).await {
        if let Err(_) = miner.auth("admin", "admin").await {
            miner.auth("root", "root").await;
        }
        miner.set_sleep(sleep).await?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("Unable to connect to miner"))
    }
}

#[derive(Serialize, Clone)]
struct SleepMiner {
    rack: i64,
    row: i64,
    index: i64,
    ip: String,
    sleep: bool,
}

async fn sleep_emit(miner: SleepMiner, client: Client, app: tauri::AppHandle) -> Result<()> {
    if let Ok(_) = set_sleep(client, &miner.ip, miner.sleep).await {
        app.emit_all("sleep", miner)?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("Unable to set sleep miner"))
    }
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
            if let miner = db::DbMiner::find_ip(db, ip).await? {
                let rack = miner.get_rack(db).await?;
                let miner = SleepMiner {
                    rack: rack.index,
                    row: miner.row,
                    index: miner.index,
                    ip: miner.ip,
                    sleep: self.sleep,
                };
                futures.push(
                    Box::pin(sleep_emit(miner, client.clone(), app.clone()))
                    as Pin<Box<dyn Future<Output = Result<()>> + Send>>
                );
            }
        }
        Ok(futures)
    }
}
