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
use crate::models::{Miner, MinerEvent};
use crate::db;
use super::JobDef;

async fn miner_pool(client: Client, ip: &str, mut pools: Pool, can: i64) -> Result<()> {
    if let Ok(mut miner) = client.get_miner(ip, None).await {
        if let Err(_) = miner.auth("admin", "admin").await {
            miner.auth("root", "root").await;
        }
        
        let mut worker = pools.username.clone();
        if worker.contains("{can}") {
            worker = worker.replace("{can}", can.to_string().as_str());
        }
        if worker.contains("{model}") {
            worker = worker.replace("{model}", &miner.get_model().await?);
        }
        if worker.contains("{ip}") {
            // Take the last 2 octets of the IP address
            let ip = ip.split('.').collect::<Vec<&str>>();
            let ip = format!("{}x{}", ip[ip.len() - 2], ip[ip.len() - 1]);
            worker = worker.replace("{ip}", &ip);
        }

        let pools = [
            libminer::Pool {
                url: pools.url1.clone(),
                username: worker.clone(),
                password: pools.password.clone(),
            },
            libminer::Pool {
                url: pools.url2.clone(),
                username: worker.clone(),
                password: pools.password.clone(),
            },
            libminer::Pool {
                url: pools.url3.clone(),
                username: worker.clone(),
                password: pools.password.clone(),
            },
        ];
    
        miner.set_pools(pools.to_vec()).await?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("Unable to connect to miner"))
    }
}

#[derive(Serialize, Clone)]
struct PoolMiner {
    can: i64,
    rack: i64,
    row: i64,
    index: i64,
    ip: String,
    pools: Pool,
}

async fn pool_emit(miner: PoolMiner, client: Client, app: tauri::AppHandle) -> Result<()> {
    if let Ok(_) = miner_pool(client, &miner.ip, miner.pools.clone(), miner.can).await {
        app.emit_all("pool", miner)?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("Unable to set locate miner"))
    }
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
            if let miner = db::DbMiner::find_ip(db, ip).await? {
                let rack = miner.get_rack(db).await?;
                let can = db::DbCan::get(db, rack.can_id).await?;

                let miner = PoolMiner {
                    can: can.num,
                    rack: rack.index,
                    row: miner.row,
                    index: miner.index,
                    ip: miner.ip,
                    pools: self.pool.clone(),
                };
                futures.push(
                    Box::pin(pool_emit(miner, client.clone(), app.clone()))
                    as Pin<Box<dyn Future<Output = Result<()>> + Send>>
                );
            }
        }
        Ok(futures)
    }
}
