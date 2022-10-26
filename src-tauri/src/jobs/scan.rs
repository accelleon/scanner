use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use tauri::AppHandle;
use libminer::Client;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use tokio::sync::oneshot;
use tokio::sync::Mutex;
use tauri::Manager;
use tracing::error;

use crate::models::{Miner, MinerEvent};
use crate::db;
use super::{Progress, JobDef};

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

async fn scan_emit(miner: &MinerScan, client: Client, app: tauri::AppHandle, progress: &Mutex<Progress>) -> Result<()>{
    let result = scan_miner(client.clone(), &miner.ip).await;
    app.emit_all("miner", MinerEvent {
        rack: miner.rack,
        row: miner.row,
        index: miner.index,
        miner: result,
    })?;
    // Scoping mutex guards is just good practice
    {
        let mut progress = progress.lock().await;
        progress.increment()?;
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct MinerScan {
    rack: i64,
    row: i64,
    index: i64,
    ip: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScanJob {
    pub can: i64,
    #[serde(skip)]
    pub todo: Vec<MinerScan>,
}

#[async_trait]
impl JobDef for ScanJob {
    async fn prepare(&mut self, db: &SqlitePool) -> Result<()> {
        let mut can = db::DbCan::get(db, self.can).await.map_err(|e| e.to_string())?;
        can.load_racks(db).await.map_err(|e| e.to_string())?;
        let mut todo = Vec::new();
        for rack in &can.racks {
            for row in &rack.miners {
                for miner in row {
                    todo.push(MinerScan {
                        rack: rack.index,
                        row: miner.row,
                        index: miner.index,
                        ip: miner.ip.clone(),
                    });
                }
            }
        }
        Ok(())
    }

    async fn run(
        &self,
        app: AppHandle,
        db: &SqlitePool,
        client: Client,
        cancel: oneshot::Receiver<()>,
        progress: &Mutex<Progress>
    ) -> Result<()> {
        let mut futures = Vec::new();
        for scan in &self.todo {
            futures.push(
                tokio::spawn(scan_emit(scan, client.clone(), app.clone(), progress))
            )
        }
        // We're gonna spawn out a separate task to wait for each JoinHandle
        let scanwait = tokio::spawn(async move {
            for future in futures {
                if let Err(e) = future.await {
                    error!("Scan future failed: {}", e);
                }
            }
        });
        // Select between the scanwait task and the cancel channel
        tokio::select! {
            _ = scanwait => {
                Ok(())
            }
            _ = cancel => {
                // Abort all join handles
                for future in futures {
                    future.abort();
                }
                Err(anyhow::format_err!("Scan cancelled"))
            }
        }
    }

    fn todo_count(&self) -> usize {
        self.todo.len()
    }
}
