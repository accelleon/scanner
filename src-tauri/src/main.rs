#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use db::DbCan;
use serde::Serialize;
use libminer::{ClientBuilder, Client};
use tokio::task::futures;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use sqlx::sqlite::SqlitePool;
use tauri::{State, Manager};
use anyhow::Result;

mod db;
mod frontier;

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

/// Import Frontier Sitemap export
#[tauri::command]
async fn import_frontier_sitemap(export: String) -> String {
    unimplemented!("Import Frontier Sitemap export");
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
        if ret.hashrate == Some(0.0) {
            ret.errors = miner.get_errors().await.unwrap_or(vec![]);
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
async fn scan_miners_async(can: i64, client: State<'_, Client>, db: State<'_, SqlitePool>, app: tauri::AppHandle) -> Result<(), String> {
    let mut can = db::DbCan::get(&db, can).await.map_err(|e| e.to_string())?;
    can.load_racks(&db).await.map_err(|e| e.to_string())?;

    let client = ClientBuilder::new()
        .build().map_err(|e| e.to_string())?;
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

#[tauri::command]
async fn scan_miners(can: i64, db: State<'_, SqlitePool>) -> Result<Vec<Rack>, String> {
    let mut can = db::DbCan::get(&db, can).await.map_err(|e| e.to_string())?;
    can.load_racks(&db).await.map_err(|e| e.to_string())?;

    let client = ClientBuilder::new()
        .build().map_err(|e| e.to_string())?;
    // Collect all ips into a list
    let mut ips = Vec::new();
    for rack in &can.racks {
        for row in &rack.miners {
            for miner in row {
                ips.push(miner.ip.clone());
            }
        }
    }
    // Scan all ips
    let mut futures = ips.iter().map(|ip| {
        tokio::spawn(scan_miner(client.clone(), ip.clone()))
    });
    // Collect all results into a map[ip] = Miner
    let mut results: HashMap<String, Miner> = HashMap::new();
    for future in futures {
        let miner = future.await.map_err(|e| e.to_string())?;
        results.insert(miner.ip.clone(), miner);
    }
    // Update all racks with the results
    Ok(can.racks.iter().map(|rack| {
        let mut res = Rack {
            name: rack.name.clone(),
            width: rack.width,
            height: rack.height,
            miners: vec![]
        };
        for row in &rack.miners {
            let mut row_res = Vec::new();
            for miner in row {
                row_res.push(results.remove(&miner.ip).unwrap());
            }
            res.miners.push(row_res);
        }
        res
    }).collect())
}

async fn main_async() {
    // Set up tracing subscriber
    tracing_subscriber::fmt::init();

    let client = ClientBuilder::new()
        .build().unwrap();

    let db = db::connect().await.unwrap();
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tauri::Builder::default()
        .manage(client)
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            get_cans,
            gen_empty_can,
            scan_miners,
            scan_miners_async,
            import_frontier_locations
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