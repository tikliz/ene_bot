use std::{path::PathBuf, io::Error, time::Duration};
use irc::client::prelude::*;
use futures::prelude::*;

#[tokio::main]
async fn main() -> irc::error::Result<()> {
    let path = PathBuf::from("./config.toml");
    let cfg = Config::load(path).unwrap();
    let mut client = Client::from_config(cfg).await?;
    client.identify()?;

    let mut stream = client.stream()?;
    let sender = client.sender();

    while let Some(message) = stream.next().await.transpose()? {
        println!("{}", message);

        match message.command {
            Command::PRIVMSG(ref target, ref msg ) => {
                if msg.starts_with('!') {
                    sender.send_privmsg(target, "101")?;

                }

            }
            _ => (),

        }

    }
    
    
    Ok(())
}
