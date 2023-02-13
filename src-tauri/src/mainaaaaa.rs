use std::{path::PathBuf, io::Error, time::Duration};
use irc::client::prelude::*;
use futures::prelude::*;
use tauri::window;

pub mod bot;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("Hello, {}! You've been greeted from Rust!", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
    
}

#[tauri::command]
async fn irc_client2() -> Result<(), ()> {
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            irc_client2,

            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    

}
