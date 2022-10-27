#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use db::DbCan;
use jobs::Job;
use serde::Serialize;
use libminer::{ClientBuilder, Client, Pool};
use sqlx::sqlite::SqlitePool;
use tauri::{State, Manager};
use anyhow::Result;
use tokio::sync::Mutex;
use tokio::sync::broadcast;

mod db;
mod frontier;
mod jobs;
mod models;
use db::Config;
use models::Can;

struct JobState {
    working: bool,
    cancel: Option<broadcast::Sender<()>>,
}

impl JobState {
    fn new() -> Self {
        Self {
            working: false,
            cancel: None,
        }
    }

    fn done(&mut self) {
        self.working = false;
        self.cancel = None;
    }

    fn start(&mut self, cancel: broadcast::Sender<()>) {
        self.working = true;
        self.cancel = Some(cancel);
    }

    async fn cancel(&mut self) -> Result<()> {
        if let Some(cancel) = self.cancel.take() {
            cancel.send(()).map_err(|_| anyhow::anyhow!("Failed to cancel job"))?;
        }
        Ok(())
    }
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
async fn run_job(
    job: Job,
    jobstate: State<'_, Mutex<JobState>>,
    client: State<'_, Mutex<Client>>,
    db: State<'_, SqlitePool>,
    app: tauri::AppHandle
) -> Result<(), String> {
    // Check if we're already working
    let mut job_guard = jobstate.lock().await;
    if job_guard.working {
        return Err("Already working".to_string());
    }

    // Set up our runner and cancel channel
    let client = client.lock().await.clone();
    let (job, cancel) = jobs::JobRunner::new(job, &db, app, client).await.map_err(|e| e.to_string())?;
    job_guard.start(cancel);
    drop(job_guard);

    // Run our job
    if let Err(e) = job.run().await {
        tracing::error!("Error running job: {}", e);
    }

    let mut job_guard = jobstate.lock().await;
    job_guard.done();
    Ok(())
}

#[tauri::command]
async fn cancel_job(jobstate: State<'_, Mutex<JobState>>) -> Result<(), String> {
    let mut job_guard = jobstate.lock().await;
    job_guard.cancel().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn get_settings(db: State<'_, SqlitePool>) -> Result<Config, String> {
    Config::load(&db).await.map_err(|e| e.to_string())
}

async fn main_async() {
    tracing_subscriber::fmt::init();

    let db = db::connect().await.unwrap();
    let config = Config::load(&db).await.unwrap();

    let jobstate = Mutex::new(JobState::new());

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
        .manage(jobstate)
        .invoke_handler(tauri::generate_handler![
            get_cans,
            gen_empty_can,
            run_job,
            cancel_job,
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
