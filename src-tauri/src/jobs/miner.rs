use tauri::{AppHandle, Manager};
use sqlx::sqlite::SqlitePool;
use anyhow::Result;

use crate::models::{MinerEvent, self};
use libminer::{Client, Profile};
use crate::db;

pub static HASH_MAP: phf::Map<&'static str, &'static str> = phf::phf_map! {
    // T19
    "NBT1903" => "240-Ca",

    // S19
    "NBS1902" => "240-Ca",
    "BHB42801" => "240-Ch",
    "BHB42831" => "240-Ch",

    // S19a
    "BHB28611" => "240-Ce",

    // S19 Pro
    "BHB42601" => "240-C",
    "BHB42651" => "j1-11",

    // S19j Pro
    "BHB42603" => "240-Cb",
    "BHB42631" => "j1-11",
};

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
    pub nameplate: Option<f64>,
    pub power: Option<f64>,
    pub efficiency: Option<f64>,
    pub profile: Option<Profile>,
    pub profiles: Option<Vec<Profile>>,
    pub hashboard: Option<String>,
    pub sleep: bool,
    pub locate: bool,
    pub client: Client,
    pub app: AppHandle,
    pub auths: db::MinerAuth,
    pub can: i64,
    pub rack: i64,
    pub row: i64,
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
            power: None,
            efficiency: None,
            profile: None,
            profiles: None,
            hashboard: None,
            sleep: false,
            locate: false,
            nameplate: None,
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
            errors: Vec::new(),
            pools: Vec::new(),
            power: None,
            efficiency: None,
            profile: None,
            profiles: None,
            hashboard: None,
            sleep: false,
            locate: false,
            nameplate: None,
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
            miner: models::Miner {
                ip: self.ip.clone(),
                make: self.make.clone(),
                model: self.model.clone(),
                submodel: self.hashboard.clone().map(|x| HASH_MAP.get(x.as_str()).map(|s| s.to_string())).unwrap_or(None),
                mac: self.mac.clone(),
                hashrate: self.hashrate,
                temp: self.temp,
                power: self.power,
                efficiency: self.efficiency,
                fan: self.fan.clone(),
                uptime: self.uptime,
                errors: self.errors.clone(),
                pools: self.pools.clone(),
                profile: self.profile.clone().map(|x| x.into()),
                profiles: self.profiles.clone().map(|x| x.into_iter().map(|x| x.into()).collect()),
                hashboard: self.hashboard.clone(),
                sleep: self.sleep,
                locate: self.locate,
                nameplate: self.nameplate,
            },
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
                if let Ok(_) = miner.auth(&auth.username, &auth.password).await {
                    authed = true;
                    break;
                }
            }
            if !authed {
                self.errors.push("Failed to auth miner".to_string());
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
            self.power = miner.get_power().await.ok();
            self.nameplate = miner.get_nameplate_rate().await.ok();
            self.efficiency = miner.get_efficiency().await.ok();
            self.profile = miner.get_profile().await.ok();
            self.profiles = miner.get_profiles().await.ok();
            self.hashboard = miner.get_hashboard().await.ok();
            // query errors if we're less than 80% of the nameplate rate
            // or if we're not hashing at all
            
            if self.hashrate.unwrap_or_else(|| unreachable!()) < 70.0 {
                for _ in 0..3 {
                    if let Ok(errors) = miner.get_errors().await.map(|r| r.into_iter().map(|e| e.msg).collect::<Vec<String>>()) {
                        self.errors = errors;
                        break;
                    }
                }
            }

            if !self.pools.is_empty() && self.pools[0].url.is_empty() {
                self.errors.push("No pool set".to_string());
            } else if self.pools.is_empty() {
                // Hackfix for the front end JS
                self.pools = vec![
                    libminer::Pool::default(),
                    libminer::Pool::default(),
                    libminer::Pool::default()
                ];
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
        let mut model = miner.get_model().await?.to_lowercase();
        // Special case for Vnish, s19-88 becomes s19
        if model.contains("-") {
            model = model.split("-").collect::<Vec<&str>>()[0].to_string();
        }
        if worker.contains("{can}") {
            worker = worker.replace("{can}", self.can.to_string().as_str());
        }
        if worker.contains("{model}") {
            worker = worker.replace("{model}", &model);
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

    pub async fn set_profile(mut self, profile: Profile) -> Result<()> {
        let mut miner = self.get_miner().await?;
        miner.set_profile(profile.into()).await?;
        Ok(self.scan().await?)
    }
}
