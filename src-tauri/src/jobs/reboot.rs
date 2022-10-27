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

async fn reboot_miner(client: Client, ip: &str) -> Result<()> {
    if let Ok(mut miner) = client.get_miner(ip, None).await {
        if let Err(_) = miner.auth("admin", "admin").await {
            miner.auth("root", "root").await;
        }
        miner.reboot().await?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("Unable to connect to miner"))
    }
}

#[derive(Serialize, Clone)]
struct RebootMiner {
    rack: i64,
    row: i64,
    index: i64,
    ip: String,
}

async fn reboot_emit(miner: RebootMiner, client: Client, app: tauri::AppHandle) -> Result<()> {
    if let Ok(_) = reboot_miner(client, &miner.ip).await {
        app.emit_all("reboot", miner)?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("Unable to set locate miner"))
    }
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
            if let miner = db::DbMiner::find_ip(db, ip).await? {
                let rack = miner.get_rack(db).await?;
                let miner = RebootMiner {
                    rack: rack.index,
                    row: miner.row,
                    index: miner.index,
                    ip: miner.ip,
                };
                futures.push(
                    Box::pin(reboot_emit(miner, client.clone(), app.clone()))
                    as Pin<Box<dyn Future<Output = Result<()>> + Send>>
                );
            }
        }
        Ok(futures)
    }
}
