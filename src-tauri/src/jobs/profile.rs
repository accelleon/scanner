use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use tauri::AppHandle;
use libminer::Client;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::pin::Pin;
use std::future::Future;

use crate::models::Profile;

use super::Miner;
use super::JobDef;

async fn set_profile(miner: Miner, profile: Profile) -> Result<()> {
    miner.set_profile(profile.into()).await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProfileJob {
    ips: Vec<String>,
    profile: Profile,
}

#[async_trait]
impl JobDef for ProfileJob {
    async fn prepare(
        &self,
        db: &SqlitePool,
        app: AppHandle,
        client: Client,
    ) -> Result<Vec<Pin<Box<dyn Future<Output = Result<()>> + Send>>>> {
        let mut futures = Vec::new();
        for ip in &self.ips {
            if let Ok(miner) = Miner::new(ip.clone(), db, client.clone(), app.clone()).await {
                futures.push(
                    Box::pin(set_profile(miner, self.profile.clone()))
                    as Pin<Box<dyn Future<Output = Result<()>> + Send>>
                );
            }
        }
        Ok(futures)
    }
}
