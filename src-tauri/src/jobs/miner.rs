use serde::Serialize;
use tauri::{AppHandle, Manager};
use sqlx::sqlite::SqlitePool;
use anyhow::Result;

use libminer::Client;
use crate::db;

#[derive(Serialize, Debug, Clone)]
pub struct MinerEvent {
    pub rack: i64,
    pub row: i64,
    pub index: i64,
    pub miner: Miner,
}

#[derive(Serialize, Debug, Clone)]
pub struct Miner {
    pub ip: String,
    pub make: Option<String>,
    pub model: Option<String>,
    pub mac: Option<String>,
    pub hashrate: Option<f64>,
    pub temp: Option<f64>,
    pub fan: Option<Vec<u32>>,
    pub uptime: Option<f64>,
    pub errors: Vec<String>,
    pub pools: Vec<libminer::Pool>,
    pub sleep: bool,
    pub locate: bool,
    #[serde(skip)]
    pub client: Client,
    #[serde(skip)]
    pub app: AppHandle,
    #[serde(skip)]
    pub auths: db::MinerAuth,
    #[serde(skip)]
    pub can: i64,
    #[serde(skip)]
    pub rack: i64,
    #[serde(skip)]
    pub row: i64,
    #[serde(skip)]
    pub index: i64,
}

impl Miner {
    pub fn default(
        ip: String,
        rack: i64,
        row: i64,
        index: i64,
        can: i64,
        app: AppHandle,
        client: Client,
        auths: db::MinerAuth,
    ) -> Self {
        Self {
            ip,
            make: None,
            model: None,
            mac: None,
            hashrate: None,
            temp: None,
            fan: None,
            uptime: None,
            errors: Vec::new(),
            pools: Vec::new(),
            sleep: false,
            locate: false,
            client,
            app,
            auths,
            can,
            rack,
            row,
            index,
        }
    }

    pub async fn new(ip: String, db: &SqlitePool, client: Client, app: AppHandle) -> Result<Self> {
        let miner = db::DbMiner::find_ip(db, &ip).await?;
        let rack = miner.get_rack(db).await?;
        let can = db::DbCan::get(db, rack.can_id).await?;
        let auths = db::MinerAuth::load(db).await?;

        Ok(Self {
            ip,
            make: None,
            model: None,
            mac: None,
            hashrate: None,
            temp: None,
            fan: None,
            uptime: None,
            errors: vec![],
            pools: vec![],
            sleep: false,
            locate: false,
            client,
            app,
            auths,
            can: can.num,
            rack: rack.index,
            row: miner.row,
            index: miner.index,
        })
    }

    pub fn emit(&self) -> Result<()> {
        let event = MinerEvent {
            rack: self.rack,
            row: self.row,
            index: self.index,
            miner: self.clone(),
        };
        self.app.emit_all("miner", event)?;
        Ok(())
    }

    pub async fn get_miner(&mut self) -> Result<Box<dyn libminer::Miner + Send + Sync>> {
        if let Ok(mut miner) = self.client.get_miner(&self.ip, None).await {
            self.make = Some(miner.get_type().to_string());
            
            let auths = self.auths.get(self.make.as_ref().unwrap());
            let mut authed = false;
            for auth in auths {
                println!("Trying auth: {:?}", auth);
                if let Ok(_) = miner.auth(&auth.username, &auth.password).await {
                    authed = true;
                    break;
                }
            }
            if !authed {
                return Err(anyhow::anyhow!("Failed to auth miner"));
            }
            Ok(miner)
        } else {
            Err(anyhow::anyhow!("Failed to get miner"))
        }
    }

    pub async fn load(&mut self) -> Result<()> {
        if let Ok(mut miner) = self.get_miner().await {
            self.model = Some(miner.get_model().await.unwrap_or("Unknown".to_string()));
            self.hashrate = Some(miner.get_hashrate().await.unwrap_or(0.0));
            self.temp = miner.get_temperature().await.ok();
            self.fan = miner.get_fan_speed().await.ok();
            self.mac = Some(miner.get_mac().await.unwrap_or("Unknown".to_string()));
            self.locate = miner.get_blink().await.unwrap_or(false);
            self.pools = miner.get_pools().await.unwrap_or(vec![]);
            if self.hashrate == Some(0.0) {
                // Try to get errors up to 3 times
                for _ in 0..3 {
                    if let Ok(errors) = miner.get_errors().await {
                        self.errors = errors;
                        break;
                    }
                }
            }
            if !self.pools.is_empty() && self.pools[0].url.is_empty() {
                self.errors.push("No pool set".to_string());
            }
            // Lastly check if miner is sleeping
            if let Ok(sleep) = miner.get_sleep().await {
                self.sleep = sleep;
            }

            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to connect to miner"))
        }
    }

    pub async fn scan(mut self) -> Result<()> {
        self.load().await?;
        Ok(self.emit()?)
    }

    pub async fn set_pool(mut self, pools: db::Pool) -> Result<()> {
        let mut miner = self.get_miner().await?;
        let mut worker = pools.username.clone();
        if worker.contains("{can}") {
            worker = worker.replace("{can}", self.can.to_string().as_str());
        }
        if worker.contains("{model}") {
            worker = worker.replace("{model}", &miner.get_model().await?.to_lowercase());
        }
        if worker.contains("{ip}") {
            // Take the last 2 octets of the IP address
            let ip = self.ip.split('.').collect::<Vec<&str>>();
            let ip = format!("{}x{}", ip[ip.len() - 2], ip[ip.len() - 1]);
            worker = worker.replace("{ip}", &ip);
        }

        let pools = vec![
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
    
        miner.set_pools(pools).await?;
        self.pools = miner.get_pools().await?;
        Ok(self.scan().await?)
    }

    pub async fn set_blink(mut self, blink: bool) -> Result<()> {
        let mut miner = self.get_miner().await?;
        miner.set_blink(blink).await?;
        Ok(self.scan().await?)
    }

    pub async fn set_sleep(mut self, sleep: bool) -> Result<()> {
        let mut miner = self.get_miner().await?;
        miner.set_sleep(sleep).await?;
        Ok(self.scan().await?)
    }

    pub async fn reboot(mut self) -> Result<()> {
        let mut miner = self.get_miner().await?;
        miner.reboot().await?;
        Ok(self.scan().await?)
    }

}