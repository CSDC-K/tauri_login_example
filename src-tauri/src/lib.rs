use std::fmt::format;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn login_validation(name : &str, password: &str) -> bool {
    println!("login_validation called in rust");
    println!("name: {}, password: {}", name, password);
    if name == "admin" && password == "password123" {
        return true;
    } else {
        return false;
    }
    false
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, login_validation])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
