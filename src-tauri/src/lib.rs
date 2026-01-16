use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize, Deserialize)]
struct LoginInfo {
    password: String
}


#[tauri::command]
fn login_validation(name : &str, password: &str) -> bool {
    println!("login_validation called in rust");
    println!("name: {}, password: {}", name, password);

    let user_data_file = fs::read_to_string("users.json").expect("Error!");
    let users: HashMap<String, LoginInfo> = serde_json::from_str(&user_data_file).expect("Error!");

    for (key, value) in users.iter() {
        if key == name && value.password == password {
            println!("Login successful for user: {}", name);
            return true;
        }
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
