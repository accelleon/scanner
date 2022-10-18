#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

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



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
