use serde::{Serialize, Deserialize};
use crate::db::DbCan;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Profile {
    Default,
    LowPower,
    Preset { name: String, power: f64, ths: f64 },
    Manual { volt: u32, freq: u32, min_freq: u32, max_freq: u32, min_volt: u32, max_volt: u32, def_freq: u32, def_volt: u32 },
} 

impl From<libminer::Profile> for Profile {
    fn from(profile: libminer::Profile) -> Self {
        match profile {
            libminer::Profile::Default => Self::Default,
            libminer::Profile::LowPower => Self::LowPower,
            libminer::Profile::Preset { name, power, ths } => Self::Preset { name, power, ths },
            libminer::Profile::Manual { volt, freq, min_freq, max_freq, min_volt, max_volt, def_freq, def_volt } => Self::Manual { volt, freq, min_freq, max_freq, min_volt, max_volt, def_freq, def_volt },
        }
    }
}

impl Into<libminer::Profile> for Profile {
    fn into(self) -> libminer::Profile {
        match self {
            Self::Default => libminer::Profile::Default,
            Self::LowPower => libminer::Profile::LowPower,
            Self::Preset { name, power, ths } => libminer::Profile::Preset { name, power, ths },
            Self::Manual { volt, freq, min_freq, max_freq, min_volt, max_volt, def_freq, def_volt } => libminer::Profile::Manual { volt, freq, min_freq, max_freq, min_volt, max_volt, def_freq, def_volt },
        }
    }
}

#[derive(Serialize)]
pub struct Can {
    id: i64,
    num: i64,
    name: String,
}

impl From<DbCan> for Can {
    fn from(can: DbCan) -> Self {
        Self {
            id: can.id,
            num: can.num,
            name: can.name,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Rack {
    pub name: String,
    pub width: i64,
    pub height: i64,
    pub miners: Vec<Vec<Miner>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Miner {
    pub ip: String,
    pub make: Option<String>,
    pub model: Option<String>,
    pub submodel: Option<String>,
    pub mac: Option<String>,
    pub hashrate: Option<f64>,
    pub temp: Option<f64>,
    pub fan: Option<Vec<u32>>,
    pub uptime: Option<f64>,
    pub errors: Vec<String>,
    pub pools: Vec<libminer::Pool>,
    pub power: Option<f64>,
    pub efficiency: Option<f64>,
    pub profile: Option<Profile>,
    pub profiles: Option<Vec<Profile>>,
    pub hashboard: Option<String>,
    pub sleep: bool,
    pub locate: bool,
    pub nameplate: Option<f64>,
}

#[derive(Serialize, Debug, Clone)]
pub struct MinerEvent {
    pub rack: i64,
    pub row: i64,
    pub index: i64,
    pub miner: Miner,
}
