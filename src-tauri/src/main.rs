use std::{path::PathBuf, io::Error, time::Duration};
use irc::client::prelude::*;
use futures::prelude::*;
use tauri::window;

mod bot;
mod main_irc;
mod irccommands;

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
        .setup(|app| {
            tauri::async_runtime::spawn(async move {
                main_irc::sus_teste_async_tauri_fodase().await;

            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet

            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    

}
