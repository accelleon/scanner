use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;

use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use tauri::AppHandle;
use libminer::Client;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use tauri::Manager;
use std::sync::Arc;

mod scan;
mod locate;
mod reboot;
mod pool;
mod sleep;

#[derive(Serialize, Debug, Clone)]
pub struct Progress {
    value: f64,
    max: usize,
    done: usize,
    job: String,
    #[serde(skip)]
    app: AppHandle,
}

impl Progress {
    fn new(app: AppHandle, job: String, max: usize) -> Self {
        Self {
            value: 0.0,
            max,
            done: 0,
            job,
            app,
        }
    }

    fn increment(&mut self) -> Result<()> {
        self.done += 1;
        self.value = self.done as f64 / self.max as f64;
        self.app.emit_all("progress", self as &Progress)?;
        Ok(())
    }

    fn emit(&self) -> Result<()> {
        self.app.emit_all("progress", self)?;
        Ok(())
    }
}

#[async_trait]
pub trait JobDef {
    /// Prepare jobs for execution
    /// This should return a list of Futures to be executed
    /// The future should emit results to the frontend
    async fn prepare(
        &self,
        db: &SqlitePool,
        app: AppHandle,
        client: Client,
    ) -> Result<Vec<Pin<Box<dyn Future<Output = Result<()>> + Send>>>>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "job")]
pub enum Job {
    Scan(scan::ScanJob),
    Locate(locate::LocateJob),
    Reboot(reboot::RebootJob),
    Pool(pool::PoolJob),
    Sleep(sleep::SleepJob),
}

impl Deref for Job {
    type Target = dyn JobDef + Send + Sync;

    fn deref(&self) -> &Self::Target {
        match self {
            Job::Scan(job) => job,
            Job::Locate(job) => job,
            Job::Reboot(job) => job,
            Job::Pool(job) => job,
            Job::Sleep(job) => job,
        }
    }
}

pub struct JobRunner {
    tasks: Vec<Pin<Box<dyn Future<Output = Result<()>> + Send>>>,
    cancel: broadcast::Sender<()>,
    progress: Arc<Mutex<Progress>>,
}

impl JobRunner {
    /// Create a new job wrapper
    /// Primarily to handle the cancellation of jobs
    /// Returns a JobRunner and a broadcast::Sender
    pub async fn new(job: Job, db: &SqlitePool, app: AppHandle, client: Client) -> Result<(Self, broadcast::Sender<()>)> {
        let tasks = job.prepare(&db, app.clone(), client).await?;
        let (cancel, _) = broadcast::channel(1);
        let progress = Arc::new(Mutex::new(Progress::new(app, "".to_string(), tasks.len())));
        Ok((Self {
            tasks,
            cancel: cancel.clone(),
            progress,
        }, cancel))
    }

    pub async fn run(self) -> Result<()> {
        self.progress.lock().await.emit();
        let mut futures = vec![];
        for task in self.tasks {
            let progress = self.progress.clone();
            let mut cancel = self.cancel.subscribe();
            futures.push(
                tokio::spawn(async move {
                    tokio::select! {
                        _ = cancel.recv() => {}
                        _ = task => {
                            progress.lock().await.increment().unwrap();
                        }
                    }
                })
            );
        }

        for future in futures {
            if let Err(e) = future.await {
                println!("Error: {}", e);
            }
        }

        Ok(())
    }
}
