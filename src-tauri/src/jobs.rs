use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use tauri::AppHandle;
use libminer::Client;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use tokio::sync::oneshot;
use tokio::sync::Mutex;
use tauri::Manager;

mod scan;

#[derive(Serialize, Debug, Clone)]
struct Progress {
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
        self.value = (self.done as f64 / self.max as f64) * 100.0;
        self.app.emit_all("progress", *self)?;
        Ok(())
    }

    fn emit(&self) -> Result<()> {
        self.app.emit_all("progress", *self)?;
        Ok(())
    }
}

#[async_trait]
pub trait JobDef {
    /// This is called prior to the job being run
    /// It should load the required data from the database
    async fn prepare(&mut self, db: &SqlitePool) -> Result<()>;

    /// Job will be passed all stateful arguments
    /// cancel is a oneshot channel that will be triggered if the job is cancelled by the user
    /// the job *must* cancel
    /// The job should increment progress as necessary
    /// The job should emit results through app.emit_all
    async fn run(
        &self,
        app: AppHandle,
        db: &SqlitePool,
        client: Client,
        cancel: oneshot::Receiver<()>,
        progress: &Mutex<Progress>
    ) -> Result<()>;

    /// This should report the total number of steps the job will take
    fn todo_count(&self) -> usize;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "job", content = "data")]
pub enum Job {
    Scan(scan::ScanJob),
}

impl Job {
    fn inner(self) -> Box<dyn JobDef + Sync + Send> {
        match self {
            Job::Scan(scan) => Box::new(scan),
        }
    }
}

pub struct JobWrapper {
    job: Job,
    recv: oneshot::Receiver<()>,
    progress: Mutex<Progress>,
}

impl JobWrapper {
    /// Create a new job wrapper
    /// Primarily to handle the cancellation of jobs
    /// Returns a JobWrapper and a oneshot::Sender
    pub fn new(job: Job, app: AppHandle) -> (Self, oneshot::Sender<()>) {
        let todo_count = job.inner().todo_count();
        let (cancel, recv) = oneshot::channel();
        let progress = Mutex::new(Progress::new(app.clone(), "".to_string(), todo_count));
        (
            Self {
                job,
                recv,
                progress,
            },
            cancel
        )
    }

    pub async fn prepare(&mut self, db: &SqlitePool) -> Result<()> {
        self.job.inner().prepare(db).await
        self.progress.lock().await.emit()
    }

    pub async fn run(self, app: AppHandle, db: &SqlitePool, client: Client) -> Result<()> {
        self.job.inner().run(app, db, client, self.recv, &self.progress).await
    }
}
