// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct FileRequest {
    path: String,
}

#[derive(Deserialize, Serialize)]
struct File {}

#[tauri::command]
fn files(path: FileRequest) -> Vec<File> {
    vec![]
}

fn main() {
    std::env::set_var("GDK_BACKEND", "x11");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
