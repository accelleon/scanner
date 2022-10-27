use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use tauri::AppHandle;
use libminer::{Client, Pool};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use tauri::Manager;
use std::pin::Pin;
use std::future::Future;

use crate::models::{Miner, MinerEvent};
use crate::db;
use super::JobDef;

async fn miner_pool(client: Client, ip: &str, mut pools: Vec<Pool>) -> Result<()> {
    if let Ok(mut miner) = client.get_miner(ip, None).await {
        if let Err(_) = miner.auth("admin", "admin").await {
            miner.auth("root", "root").await;
        }
        let mut model = None;

        // Replace any miner specific placeholders
        // Ensure to only query the miner once for necessary values
        for pool in &mut pools {
            if pool.username.contains("{model}") {
                let m = match model {
                    Some(_) => model.as_ref().unwrap(),
                    None => {
                        let m = miner.get_model().await?;
                        model = Some(m);
                        model.as_ref().unwrap()
                    }
                };
                pool.username = pool.username.replace("{model}", &m);
            }
        }
    
        miner.set_pools(pools).await?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("Unable to connect to miner"))
    }
}

#[derive(Serialize, Clone)]
struct PoolMiner {
    rack: i64,
    row: i64,
    index: i64,
    ip: String,
    pools: Vec<Pool>,
}

async fn reboot_emit(miner: PoolMiner, client: Client, app: tauri::AppHandle) -> Result<()> {
    if let Ok(_) = miner_pool(client, &miner.ip, miner.pools.clone()).await {
        app.emit_all("pool", miner)?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("Unable to set locate miner"))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetPool {
    pub url: String,
    pub user: String,
    pub pass: Option<String>,
    pub ip_suffix: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoolJob {
    ips: Vec<String>,
    pools: Vec<SetPool>,
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

                let mut pools = Vec::new();
                for pool in &self.pools {
                    let mut user = pool.user.clone();
                    // Append IP suffix if we have one
                    if pool.ip_suffix > 0 {
                        let octets = ip.split('.').collect::<Vec<&str>>();
                        let caps = octets.as_slice()[octets.len() - pool.ip_suffix as usize..].to_vec().join("x");
                        // If the user doesn't end with a dot add one
                        if !user.ends_with('.') {
                            user.push('.');
                        }
                        user.push_str(&caps);
                    }

                    // Replace any placeholders
                    user = user.replace("{can}", &can.name);

                    pools.push(Pool {
                        url: pool.url.clone(),
                        username: user,
                        password: pool.pass.clone(),
                    });
                }

                let miner = PoolMiner {
                    rack: rack.index,
                    row: miner.row,
                    index: miner.index,
                    ip: miner.ip,
                    pools,
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
