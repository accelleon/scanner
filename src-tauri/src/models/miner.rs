use serde::Serialize;
use crate::db::DbCan;
use crate::jobs::Miner;

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
pub struct MinerEvent {
    pub rack: i64,
    pub row: i64,
    pub index: i64,
    pub miner: Miner,
}
