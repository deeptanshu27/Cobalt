// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn run(name: &str) -> String {
    // let _spawn = std::process::Command::new(name).spawn();
    let _result = std::process::Command::new("cmd")
            .args(&["/C", "start", "", name])
            .spawn();
    format!("<3")
}

#[tauri::command]
fn settings(name: &str) -> String {
    let _status = std::process::Command::new("notepad")
      .arg(name)
      .status()
      .expect("Failed to start Notepad");
    format!("<3")
}

use serde_json::{Value, from_str, to_string_pretty};
use std::fs;

#[tauri::command]
fn add(loc: &str, id: &str, name: &str, img: &str, shortcut: &str) -> String {
    // chatgpt moment:
    // Read the file content
    let data = fs::read_to_string(loc).unwrap();

    // Parse the JSON content
    let mut json_data: Value = from_str(&data).unwrap();

    // Print the JSON data to see its current state (for debugging purposes)
    // println!("Original data: {}", to_string_pretty(&json_data).unwrap());

    // Modify the JSON data
    let new_data = serde_json::json!({id: {"name": name, "img": img, "loc": shortcut}});
    if let Some(object) = json_data.as_object_mut() {
        object.extend(new_data.as_object().unwrap().clone());
    }

    // Write the updated data back to the file
    let updated_data = to_string_pretty(&json_data).unwrap();
    let _ = fs::write(loc, updated_data);

    format!("<3")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run, settings, add])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
