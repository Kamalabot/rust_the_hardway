// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{AppHandle, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command] // this makes the below function into tauri command
fn greet(app: AppHandle, name: &str) -> String {
    app.emit_all("event", "eventPayload").unwrap();
    format!("Hello, {}! You've been greeted from Rust through JS!", name)
}

#[tauri::command] // this makes the below function into tauri command
fn greet_more(name: &str) -> String {
    format!("Hello, {}! Lets get more greetings", name)
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, greet_more])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // this runs the backend and sends info to the front, when invoked
}
