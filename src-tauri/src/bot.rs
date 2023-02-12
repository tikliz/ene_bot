use futures::prelude::*;
use irc::client::{prelude::*, ClientStream};
use tokio::time::sleep;
//use serde_json::error;
//use tokio::time::error::Elapsed;
use std::{path::PathBuf, error::Error, time::Duration};
mod irccommands;

struct Handler;

impl Handler {
    pub fn ready(sender: &Sender, target: &String, msg: &String, client: &Client)
    {
        let response: Option<String> = match msg.split_whitespace().next().unwrap()
        {
            "!request" => irccommands::request::run(msg),
            "!help" => irccommands::help::run(msg),
            "!q" => irccommands::leave::run(client),
            _ => None,

        };
        if let Some(m) = response 
        {
            
            let a = sender.send_privmsg(target, m);
            match a {
                Ok(()) => {},
                Err(_) => {},
            };
            println!("{:#?}", a);

        }
        

    }
}

enum ClientInfo {
    ClientEn(Client),


}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    
    let mut client: Client;
    let mut stream: ClientStream;
    loop {
        if let (Some(asc_client), Some(asc_stream)) = irccommands::test::connect_client().await {
            client = asc_client;
            stream = asc_stream;
            break;

        }
        tokio::time::sleep(Duration::from_secs(1)).await;

    }

    //let mut stream = client.stream()?;
    let sender = client.sender();


    println!("connected as: {}", client.current_nickname());
    //println!("{:#?}", stream);
    if stream.next().await.transpose()? == None {
        println!("FAILED TO CONNECT.");

    }
    client.send(Command::PING("tmi.twitch.tv".into(), None)).unwrap();
    while let Some(message) = stream.next().await.transpose()?
    {
        // let message = match stream.next().await.transpose()
        // {
        //     Ok(v) => v,
        //     Err(e) => panic!("5 unwrap resulted in {}", e),
        // };

        
        //print!("{:#?}", message);

        match message.command {
            Command::PING(ref target, ref msg) => {
                println!("PING RECEIVED I THINK");

            }
            Command::PONG(ref target, ref msg) => {
                println!("PONG RECEIVED");

            }
            Command::NOTICE(ref target, ref msg) => {
                println!("NOTICE RECEIVED I THINK");

            }
            Command::PRIVMSG(ref target, ref msg) => {
                Handler::ready(&sender, target, msg, &client);
            }
            _ => (),
        }
    }
    //let message = stream.next().await.transpose()?;
    print!("done.");

    /*
    fn handle_bot_checkbox(checked: bool) {
        if checked {
            println!("blabla");
            
        }

    }
    */


    



    Ok(())
}