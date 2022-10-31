use serde::Deserialize;
use anyhow::Result;
use csv;
use sqlx::sqlite::SqlitePool;
use tracing::info;

use crate::db::{DbRack, DbMiner, DbCan};

#[derive(Deserialize)]
struct SitemapRecord {
    pickaxe_id: String,
    miner_ip: String,
    miner_port: u16,
    rack: String,
    row: i64,
    index: i64,
}

#[derive(Deserialize)]
enum RackRecordType {
    #[serde(rename = "group")]
    GROUP,
    #[serde(rename = "rack")]
    RACK,
}

#[derive(Deserialize)]
struct RackRecord {
    group_name: Option<String>,
    #[serde(rename = "type")]
    type_: RackRecordType,
    name: String,
    row: i64,
    column: i64,
    rack_width: Option<i64>,
    rack_height: Option<i64>,
}

pub async fn import_sitemap(db: &SqlitePool, layout: String, sitemap: String) -> Result<()> {
    // Drop all data in existing tables
    sqlx::query("DELETE FROM miners").execute(db).await?;
    sqlx::query("DELETE FROM racks").execute(db).await?;
    sqlx::query("DELETE FROM cans").execute(db).await?;

    print!("Layout file: {}", layout);
    print!("Sitemap file: {}", sitemap);

    info!("Importing sitemap");
    // Parse layout csv file first
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(layout)?;
    // let mut cans = Vec::new();
    // let mut racks = Vec::new();

    for result in rdr.deserialize::<RackRecord>() {
        let record = result?;
        match record.type_ {
            RackRecordType::GROUP => {
                info!("Group: {}", record.name);
                sqlx::query!(r#"
                    INSERT INTO cans (name, num)
                    VALUES (?, ?)
                    "#,
                    record.name,
                    record.row,
                ).execute(db).await?;
            },
            RackRecordType::RACK => {
                let group_name = record.group_name.unwrap();
                let row = sqlx::query!(r#"
                    SELECT id FROM cans WHERE name = ?
                    "#,
                    group_name,
                ).fetch_one(db).await?;
                info!("Rack: {} in {}", record.name, group_name);
                let width = record.rack_width.unwrap();
                let height = record.rack_height.unwrap();
                sqlx::query!(r#"
                    INSERT INTO racks (name, index_, width, height, can_id)
                    VALUES (?, ?, ?, ?, ?)
                    "#,
                    record.name,
                    record.column,
                    width,
                    height,
                    row.id
                ).execute(db).await?;
            },
        }
    }

    // Parse sitemap csv file
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(sitemap)?;
    for result in rdr.deserialize::<SitemapRecord>() {
        let record = result?;
        info!("Miner: {} at {} {} {}", record.miner_ip, record.rack, record.row, record.index);
        let rack = sqlx::query!(r#"
            SELECT id FROM racks WHERE name = ?
            "#,
            record.rack
        ).fetch_one(db).await?;
        sqlx::query!(r#"
            INSERT INTO miners (ip, rack_id, row, index_)
            VALUES (?, ?, ?, ?)
            "#,
            record.miner_ip,
            rack.id,
            record.row,
            record.index
        ).execute(db).await?;
    }

    Ok(())
}
