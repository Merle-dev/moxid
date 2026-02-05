// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct File {
    name: String,
    directory: bool,
}

#[tauri::command]
fn files(path: String) -> Result<Vec<File>, String> {
    println!("File from {path}");
    let files: Result<Vec<File>, String> = std::fs::read_dir(&path)
        .map_err(|_| format!("Failed to read directory: {path}"))?
        .map(|entry_result| {
            let entry = entry_result.map_err(|e| format!("OI Error: {e}"))?;
            let name = entry
                .file_name()
                .into_string()
                .map_err(|e| format!("Invalid filename: {:?}", e))?;

            let directory = entry
                .metadata()
                .map_err(|_| format!("Failed to get metadata for {}", name))?
                .is_dir();

            Ok(File { name, directory })
        })
        .collect();
    let sort = |mut v: Vec<File>| {
        v.sort_by(compare);
        v
    };
    files.map(sort)
}

#[tauri::command]
fn directory() -> Result<String, String> {
    std::env::current_dir()
        .map(|p| p.display().to_string())
        .map_err(|e| format!("{e}"))
}

fn compare(a: &File, b: &File) -> Ordering {
    b.directory.cmp(&a.directory).then(a.name.cmp(&b.name))
}

#[tauri::command]
fn cc(invoke_message: String) {
    println!(
        "I was invoked from JavaScript, with this message: {}",
        invoke_message
    );
}

fn main() {
    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    unsafe {
        std::env::set_var("GDK_BACKEND", "x11");
    }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![files, directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
