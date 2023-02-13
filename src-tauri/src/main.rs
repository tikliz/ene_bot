//use serde_json::error;
//use tokio::time::error::Elapsed;
use std::{error::Error, path::PathBuf}; // time::Duration
use irc::{
    client::{prelude::*, ClientStream},
    proto::response,
};

mod bot;
mod irccommands;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // passar essas coisas por variavel de ambiente (eventualmente)
    let config = Config::load(PathBuf::from("./../../config.toml"))
        .expect("<fatal: config file is broken or not found>");

        
    let handler_foda = bot::Handler::new([
        bot::CommandRegister {
            command: "!request".to_string(),
            description: "description foda".to_string(),
            usage: "!request [URL] [message]".to_string(),
            run: irccommands::request::run
        },
        bot::CommandRegister {
            command: "!help".to_string(),
            description: "description".to_string(),
            usage: "!help [command]".to_string(),
            run: irccommands::help::run
        },
        bot::CommandRegister {
            command: "!q".to_string(),
            description: "i die thank you foreva".to_string(),
            usage: "!q".to_string(),
            run: irccommands::help::run,

        }
    ]
    .to_vec());
    
    // refatorar esse stop talvez
    let mut run_bot = true;
    // se pa da pra jogar no tauri isso aqui ja
    while run_bot {
        let mut instanciadobotircmuitofoda = bot::Irc::new(config.clone()).await;
        run_bot = instanciadobotircmuitofoda.run(&handler_foda).await;

    }

    Ok(())
}
