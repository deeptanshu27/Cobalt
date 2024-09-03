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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run, settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
