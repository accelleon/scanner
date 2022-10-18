#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use libminer::{ClientBuilder, Client};
mod db;

struct IRack {
    ips: Vec<Vec<&'static str>>,
}

struct Can {
    racks: Vec<IRack>,
}

fn get_can() -> Can {
    Can {
        racks: vec![
            IRack {
                ips: vec![
                    vec![
                        "10.20.0.1",
                        "10.20.0.2",
                        "10.20.0.3",
                        "10.20.0.4",
                    ],
                    vec![
                        "10.20.0.5",
                        "10.20.0.6",
                        "10.20.0.7",
                        "10.20.0.8",
                    ],
                    vec![
                        "10.20.0.9",
                        "10.20.0.10",
                        "10.20.0.11",
                        "10.20.0.12",
                    ],
                    vec![
                        "10.20.0.13",
                        "10.20.0.14",
                        "10.20.0.15",
                        "10.20.0.16",
                    ],
                    vec![
                        "10.20.0.17",
                        "10.20.0.18",
                        "10.20.0.19",
                        "10.20.0.20",
                    ],
                    vec![
                        "10.20.0.21",
                        "10.20.0.22",
                        "10.20.0.23",
                        "10.20.0.24",
                    ],
                ],
            },
        ]
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Import Brightly export
#[tauri::command]
async fn import_brightly(export: String) -> String {
    unimplemented!("Import Brightly export");
}

/// Import Frontier Locations export
#[tauri::command]
async fn import_frontier_locations(export: String) -> String {
    unimplemented!("Import Frontier Locations export");
}

/// Import Frontier Sitemap export
#[tauri::command]
async fn import_frontier_sitemap(export: String) -> String {
    unimplemented!("Import Frontier Sitemap export");
}

#[derive(Serialize, Debug)]
struct Miner {
    ip: String,
    make: Option<String>,
    model: Option<String>,
    hashrate: Option<f64>,
}

#[derive(Serialize, Debug)]
struct Rack {
    name: String,
    miners: Vec<Vec<Miner>>,
}

#[tauri::command]
async fn scan_miners(location: String) -> Vec<Rack> {
    // C24-1
    //let racks = locations[location].racks;
    //for rack in racks: {
        // let miners = vec![];
        // for i, ip in enumerate(rack) {
            // let row = vec![];
            // for j, ip in enumerate(ip) {
                // let miner = Miner {
                    // ip: ip,
                    // make: None,
                    // model: None,
                    // hashrate: None,
                // };
                // row.push(miner);
            // }
        //}
    //}

    let client = ClientBuilder::new()
        .build()
        .unwrap();

    let mut miners = vec![];

    for rack in get_can().racks {
        let mut miner_rack = Rack {
            name: "C24-1".to_string(),
            miners: vec![],
        };
        for row in rack.ips {
            let mut miner_row = vec![];
            for ip in row {
                let miner = client.get_miner(&ip, None).await;
                if let Ok(mut miner) = miner {
                    if let Err(_) = miner.auth("admin", "admin").await {
                        miner.auth("root", "root").await;
                    }
                    miner_row.push(Miner {
                        ip: ip.to_string(),
                        make: Some(miner.get_type().to_string()),
                        model: Some(miner.get_model().await.unwrap_or("Unknown".to_string())),
                        hashrate: Some(miner.get_hashrate().await.unwrap_or(0.0)),
                    });
                } else {
                    miner_row.push(Miner {
                        ip: ip.to_string(),
                        make: None,
                        model: None,
                        hashrate: None,
                    });
                }
            }
            miner_rack.miners.push(miner_row);
        }
        miners.push(miner_rack);
    }
    
    miners
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan_miners])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
