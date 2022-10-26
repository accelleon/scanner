#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use db::DbCan;
use serde::{Serialize, Deserialize};
use libminer::{ClientBuilder, Client, Pool};
use sqlx::sqlite::SqlitePool;
use tauri::{State, Manager};
use anyhow::Result;
use tokio::sync::Mutex;

mod db;
mod frontier;
use db::Config;

#[derive(Serialize)]
struct Can {
    id: i64,
    name: String,
}

impl From<DbCan> for Can {
    fn from(can: DbCan) -> Self {
        Self {
            id: can.id,
            name: can.name,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
struct Miner {
    ip: String,
    make: Option<String>,
    model: Option<String>,
    mac: Option<String>,
    hashrate: Option<f64>,
    temp: Option<f64>,
    fan: Option<Vec<u32>>,
    uptime: Option<f64>,
    errors: Vec<String>,
    pools: Vec<Pool>,
    sleep: bool,
}

#[derive(Serialize, Debug)]
struct Rack {
    name: String,
    width: i64,
    height: i64,
    miners: Vec<Vec<Miner>>,
}

#[derive(Serialize, Debug, Clone)]
struct MinerEvent {
    rack: i64,
    row: i64,
    index: i64,
    miner: Miner,
}

#[derive(Serialize, Debug, Clone)]
struct Progress {
    value: f64,
    max: usize,
    done: usize,
    job: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    refreshRate: u64,
    maxConnections: usize,
    connectionTimeout: u64,
    readTimeout: u64,
}

impl Config {
    fn new() -> Self {
        Self {
            refreshRate: 30,
            maxConnections: 500,
            connectionTimeout: 10,
            readTimeout: 15,
        }
    }

    async fn load(db: &SqlitePool) -> Result<Self> {
        let row = sqlx::query!("SELECT * FROM config")
            .fetch_one(db)
            .await;
        match row {
            Err(sqlx::Error::RowNotFound) | Err(sqlx::Error::Database(_)) => {
                let default = Config::new();
                sqlx::query!("CREATE TABLE IF NOT EXISTS config (
                    id INTEGER PRIMARY KEY NOT NULL,
                    key TEXT NOT NULL UNIQUE,
                    value TEXT NOT NULL
                );")
                    .execute(db)
                    .await?;
                default.save(db).await?;
                Ok(default)
            }
            Ok(row) => {
                Ok(serde_json::from_str(&row.value)?)
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn save(&self, db: &SqlitePool) -> Result<()> {
        let serial = serde_json::to_string(self)?;
        sqlx::query!("UPDATE config SET value = ? WHERE key = 'json'",
            serial
        )
            .execute(db)
            .await?;
        Ok(())
    }
}

async fn update_progress(job: String, done: usize, max: usize, app: tauri::AppHandle) -> Result<()> {
    let progress = Progress {
        value: done as f64 / max as f64,
        max,
        done,
        job,
    };
    Ok(app.emit_all("progress", progress)?)
}

#[tauri::command]
async fn get_cans(db: State<'_, SqlitePool>) -> Result<Vec<Can>, String> {
    let cans = db::DbCan::all(&db).await.map_err(|e| e.to_string())?;
    Ok(cans.into_iter().map(|can| can.into()).collect())
}

#[tauri::command]
async fn gen_empty_can(can: i64, db: State<'_, SqlitePool>) -> Result<DbCan, String> {
    let mut can = db::DbCan::get(&db, can).await.map_err(|e| e.to_string())?;
    can.load_racks(&db).await.map_err(|e| e.to_string())?;
    Ok(can)
}

/// Import Frontier Locations export
#[tauri::command]
async fn import_frontier_locations(layout: String, sitemap: String, db: State<'_, SqlitePool>) -> Result<(), ()> {
    if let Err(e) = frontier::import_sitemap(&db, layout, sitemap).await {
        tracing::error!("Error importing sitemap: {}", e);
    }
    Ok(())
}

#[tauri::command]
async fn save_settings(settings: Config, client: State<'_, Mutex<Client>>, db: State<'_, SqlitePool>) -> Result<(), String> {
    settings.save(&db).await.map_err(|e| e.to_string())?;
    let new_client = ClientBuilder::new()
        .max_connections(settings.maxConnections)
        .connect_timeout(tokio::time::Duration::from_secs(settings.connectionTimeout))
        .request_timeout(tokio::time::Duration::from_secs(settings.readTimeout))
        .build()
        .map_err(|e| e.to_string())?;
    let mut client = client.lock().await;
    *client = new_client;
    Ok(())
}

#[tauri::command]
async fn get_settings(db: State<'_, SqlitePool>) -> Result<Config, String> {
    Config::load(&db).await.map_err(|e| e.to_string())
}

async fn scan_miner(client: Client, ip: String) -> Miner {
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
    if let Ok(mut miner) = client.get_miner(&ip, None).await {
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

async fn scan_emit(rack: i64, row: i64, index: i64, ip: String, client: Client, app: tauri::AppHandle) {
    let miner = scan_miner(client.clone(), ip).await;
    app.emit_all("miner", MinerEvent {
        rack,
        row,
        index,
        miner,
    });
}

#[tauri::command]
async fn scan_miners_async(can: i64, client: State<'_, Mutex<Client>>, db: State<'_, SqlitePool>, app: tauri::AppHandle) -> Result<(), String> {
    let mut can = db::DbCan::get(&db, can).await.map_err(|e| e.to_string())?;
    can.load_racks(&db).await.map_err(|e| e.to_string())?;

    let client = client.lock().await.clone();

    // Collect all ips into a list
    let mut futures = Vec::new();
    for rack in &can.racks {
        for row in &rack.miners {
            for miner in row {
                futures.push(
                    tokio::spawn(scan_emit(rack.index, miner.row, miner.index, miner.ip.clone(), client.clone(), app.clone()))
                )
            }
        }
    }
    let mut done: usize = 0;
    let max = futures.len();
    update_progress("Scanning".to_string(), done, max, app.clone()).await;
    for future in futures {
        future.await;
        done += 1;
        update_progress("Scanning...".to_string(), done, max, app.clone()).await;
    }
    Ok(())
}

async fn main_async() {
    // Set up tracing subscriber
    tracing_subscriber::fmt::init();

    let db = db::connect().await.unwrap();

    let config = Config::load(&db).await.unwrap();

    let client = ClientBuilder::new()
        .connect_timeout(tokio::time::Duration::from_secs(config.connectionTimeout))
        .request_timeout(tokio::time::Duration::from_secs(config.readTimeout))
        .max_connections(config.maxConnections)
        .build()
        .unwrap();

    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tauri::Builder::default()
        .manage(Mutex::new(client))
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            get_cans,
            gen_empty_can,
            scan_miners_async,
            import_frontier_locations,
            save_settings,
            get_settings,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            main_async().await;
        });
}
