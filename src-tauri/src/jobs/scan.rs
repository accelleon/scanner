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

async fn scan_miner(client: Client, ip: &str) -> Miner {
    let mut ret = Miner {
        ip: ip.to_string(),
        make: None,
        model: None,
        hashrate: None,
        temp: None,
        fan: None,
        uptime: None,
        mac: None,
        errors: vec![],
        pools: vec![],
        sleep: false,
        locate: false,
    };
    if let Ok(mut miner) = client.get_miner(ip, None).await {
        ret.make = Some(miner.get_type().to_string());
        if let Err(_) = miner.auth("admin", "admin").await {
            miner.auth("root", "root").await;
        }
        ret.model = Some(miner.get_model().await.unwrap_or("Unknown".to_string()));
        ret.hashrate = Some(miner.get_hashrate().await.unwrap_or(0.0));
        ret.temp = miner.get_temperature().await.ok();
        ret.fan = miner.get_fan_speed().await.ok();
        ret.mac = Some(miner.get_mac().await.unwrap_or("Unknown".to_string()));
        ret.locate = miner.get_blink().await.unwrap_or(false);
        ret.pools = miner.get_pools().await.unwrap_or(vec![]);
        if ret.hashrate == Some(0.0) {
            // Try to get errors up to 3 times
            for _ in 0..3 {
                if let Ok(errors) = miner.get_errors().await {
                    ret.errors = errors;
                    break;
                }
            }
        }
        if !ret.pools.is_empty() && ret.pools[0].url.is_empty() {
            ret.errors.push("No pool set".to_string());
        }
        // Lastly check if miner is sleeping
        if let Ok(sleep) = miner.get_sleep().await {
            ret.sleep = sleep;
        }
    }
    ret
}

async fn scan_emit(miner: MinerScan, client: Client, app: tauri::AppHandle) -> Result<()>{
    let result = scan_miner(client.clone(), &miner.ip).await;
    app.emit_all("miner", MinerEvent {
        rack: miner.rack,
        row: miner.row,
        index: miner.index,
        miner: result,
    })?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct MinerScan {
    rack: i64,
    row: i64,
    index: i64,
    ip: String,
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
        let mut futures = vec![];
        for rack in &can.racks {
            for row in &rack.miners {
                for miner in row {
                    let miner = MinerScan {
                        rack: rack.index,
                        row: miner.row,
                        index: miner.index,
                        ip: miner.ip.clone(),
                    };
                    futures.push(
                        Box::pin(scan_emit(miner, client.clone(), app.clone()))
                        as Pin<Box<dyn Future<Output = Result<()>> + Send>>
                    );
                }
            }
        }
        Ok(futures)
    }
}
