use std::{path::PathBuf, io::Error, time::Duration};
use irc::client::prelude::*;
use futures::prelude::*;
use tauri::{window, Manager};
use tokio::sync::mpsc;

pub mod irccommands;
pub mod main_irc;
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

pub async fn send_request_tauri<R: tauri::Runtime>(msg: String, manager: &impl Manager<R>) {
    let request = "teste".to_string();
    manager.emit_all("ADD_TO_LIST", msg).unwrap();
    
}

fn main() {  
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            // tauri::async_runtime::spawn(async move {
            //     loop {
            //         if let Some(output) = async_process_output_rx.recv().await {
            //             send_request_tauri(output, &app_handle);
            //         }
            //     }
            // });
            let main_window = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                main_irc::sus_teste_async_tauri_fodase(main_window).await.unwrap();
                
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,

            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    

}