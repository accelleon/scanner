use serde::Serialize;
use crate::db::DbCan;

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
    pub mac: Option<String>,
    pub hashrate: Option<f64>,
    pub temp: Option<f64>,
    pub fan: Option<Vec<u32>>,
    pub uptime: Option<f64>,
    pub errors: Vec<String>,
    pub pools: Vec<libminer::Pool>,
    pub power: Option<f64>,
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
