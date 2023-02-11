use std::{path::PathBuf, io::Error, time::Duration};
use irc::client::prelude::*;
use futures::prelude::*;
use tauri::window;

mod bot;

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
    let path = PathBuf::from("../config.toml");
    let cfg = Config::load(path).unwrap();
    let mut client = Client::from_config(cfg).await.unwrap();
    client.identify().unwrap();

    let mut stream = client.stream().unwrap();
    let sender = client.sender();

    while let Some(message) = stream.next().await.transpose().unwrap() {
        println!("{}", message);

        match message.command {
            Command::PRIVMSG(ref target, ref msg ) => {
                if msg.starts_with('!') {
                    sender.send_privmsg(target, "101").unwrap();

                }

            }
            _ => (),

        }
        

    }
    fn handle_bot_checkbox(checked: bool) {
        if checked {
            println!("blabla");
            
        }

    }
    
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
