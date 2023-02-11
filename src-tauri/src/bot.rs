use std::{path::PathBuf, io::Error, time::Duration};
use irc::client::prelude::*;
use futures::prelude::*;

#[tauri::command]
pub async fn irc_client() -> Result<(), ()> {
    let path = PathBuf::from("./config.toml");
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
    
    
    Ok(())
}
