//use serde_json::error;
//use tokio::time::error::Elapsed;
use std::{error::Error, path::PathBuf, rc::Rc, any::Any}; // time::Duration
use irc::{
    client::{prelude::*, ClientStream},
    proto::response,
};
use tauri::Window;

use crate::bot;
use crate::irccommands;

pub async fn sus_teste_async_tauri_fodase(window: Window) -> Result<(), Box<dyn Error>> {
    // passar essas coisas por variavel de ambiente (eventualmente)
    let config = Config::load(PathBuf::from("./config.toml"))
        .expect("<fatal: config file is broken or not found>");

        
    let handler_foda = bot::Handler::new([
        bot::CommandRegister {
            window: Some(window.clone()),
            command: None,
            tag: Some("REQ".to_string()),
            description: "channel points request".to_string(),
            usage: "[request with points] [URL] [message]".to_string(),
            run: irccommands::request::run

        },
        bot::CommandRegister {
            window: Some(window.clone()),
            command: Some("!request".to_string()),
            tag: None,
            description: "description foda".to_string(),
            usage: "!request [URL] [message]".to_string(),
            run: irccommands::request_normal::run
        },
        bot::CommandRegister {
            window: None,
            command: Some("!help".to_string()),
            tag: None,
            description: "description".to_string(),
            usage: "!help [command]".to_string(),
            run: irccommands::help::run
        },
        bot::CommandRegister {
            window: None,
            command: Some("!q".to_string()),
            tag: None,
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