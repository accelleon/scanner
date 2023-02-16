use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use tauri::AppHandle;
use libminer::Client;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::pin::Pin;
use std::future::Future;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::db;
use super::JobDef;
use super::Miner;

async fn log(ip: String, client: Client, auths: db::MinerAuth, folder: String) -> Result<()> {
    if let Ok(mut miner) = client.get_miner(&ip, None).await {
        let make = miner.get_type().to_string();
        
        let auths = auths.get(&make);
        let mut authed = false;
        for auth in auths {
            if let Ok(_) = miner.auth(&auth.username, &auth.password).await {
                authed = true;
                break;
            }
        }
        if !authed {
            return Err(anyhow::anyhow!("Failed to auth miner"));
        }
        
        let mac = miner.get_mac().await?.to_lowercase().replace(":", ".");
        let logs = miner.get_logs().await?.join("\n");

        let mut file = File::create(format!("{}\\{}.log", folder, mac)).await?;
        file.write_all(logs.as_bytes()).await?;
        file.flush().await?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to get miner"))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogJob {
    pub ips: Vec<String>,
    pub path: String,
}

#[async_trait]
impl JobDef for LogJob {
    async fn prepare(
        &self,
        db: &SqlitePool,
        app: AppHandle,
        client: Client,
    ) -> Result<Vec<Pin<Box<dyn Future<Output = Result<()>> + Send>>>> {
        let auths = db::MinerAuth::load(db).await?;
        let mut futures = vec![];
        for ip in &self.ips {
            futures.push(
                Box::pin(log(ip.clone(), client.clone(), auths.clone(), self.path.clone()))
                as Pin<Box<dyn Future<Output = Result<()>> + Send>>
            );
        }
        Ok(futures)
    }
}
