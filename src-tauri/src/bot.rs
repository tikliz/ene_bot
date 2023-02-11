use futures::prelude::*;
use irc::client::prelude::*;
//use serde_json::error;
//use tokio::time::error::Elapsed;
use std::path::PathBuf;
mod irccommands;

struct Handler;

impl Handler {


    

    pub fn ready(sender: &Sender, target: &String, msg: &String)
    {
        let response: Option<String> = match msg.split_whitespace().next().unwrap()
        {
            "!request" => irccommands::request::run(msg),
            "!help" => irccommands::help::run(msg),
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



#[tokio::main]
async fn main() -> irc::error::Result<()> {
    /*
    let config = Config {
        nickname: Some("pickles".to_owned()),
        server: Some("chat.freenode.net".to_owned()),
        channels: vec!["#rust-spam".to_owned()],
        ..Default::default()
    };
    */

    let path = PathBuf::from("./config.toml");
    let config = match Config::load(path)
    {
        Ok(v) => v,
        Err(e) => panic!("1 unwrap resulted in {}", e),
    };

    
    let mut client = match Client::from_config(config).await
    {
        Ok(v) => v,
        Err(e) => panic!("2 unwrap resulted in {}", e),
    };

    match client.identify()
    {
        Ok(v) => v,
        Err(e) => panic!("3 unwrap resulted in {}", e),
    };
    
    let mut stream = match client.stream()
    {
        Ok(v) => v,
        Err(e) => panic!("4 unwrap resulted in {}", e),
    };

    let sender = client.sender();



    //client.send_quit("asdsad").unwrap();

    //while let Some(message) = stream.next().await.transpose()?
    loop
    {

        let message = match stream.next().await.transpose()
        {
            Ok(v) => v,
            Err(e) => panic!("5 unwrap resulted in {}", e),
        };

        
        print!("{:#?}", message);

        match message.unwrap().command {
            Command::PRIVMSG(ref target, ref msg) => {
                Handler::ready(&sender, target, msg);
            }
            _ => (),
        }
    }

    print!("HAHAHAHAHAA");

    /*
    fn handle_bot_checkbox(checked: bool) {
        if checked {
            println!("blabla");
            
        }

    }
    */


    



    Ok(())
}