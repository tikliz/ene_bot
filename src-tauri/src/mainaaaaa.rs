//use serde_json::error;
//use tokio::time::error::Elapsed;
use std::{error::Error, path::PathBuf}; // time::Duration
mod irccommands;
use irc::{
    client::{prelude::*, ClientStream},
    proto::response,
};

pub mod bot;

// precisa de uns dar uma trabalhada nisso aqui, tenho umas ideias pode deixar que eu faco isso
pub fn commandshandler(bot: &mut bot::Irc, target: &String, msg: &String) {
    // nao e um bom lugar pra fazer isso aqui
    let help = irccommands::help::create(
        [
            irccommands::help::Help {
                command: "!request".to_string(),
                description: "description foda".to_string(),
                usage: "!request [URL]".to_string(),
            },
            irccommands::help::Help {
                command: "!help".to_string(),
                description: "description".to_string(),
                usage: "!help [command]".to_string(),
            },
        ]
        .to_vec(),
    );

    // ele ta splitando todas as mensagens inteiras precisa mudar essa forma de split
    let response: Option<String> = match msg.split_whitespace().next().unwrap() {
        "!request" => {
            println!("<info: received command \"{}\">", msg);
            irccommands::request::run(msg)
        }
        "!help" => {
            println!("<info: received command \"{}\">", msg);
            irccommands::help::run(msg, &help)
        }
        _ => None,
    };
    if let Some(resp) = response {
        let send_response = bot.sender.send_privmsg(target, resp);
        match send_response {
            Ok(()) => {
                println!("<info: response sent \"{}\">", "response.unwrap()");
            }
            Err(_) => {
                println!("<error: failed to send response \"{:#?}\">", send_response);
            }
        };
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // talvez passar essas coisas por variavel de ambiente e a forma mais segura de se fazer isso e amais comum de ver em codigo alheio
    let config = Config::load(PathBuf::from("../config.toml"))
        .expect("<fatal: config file is broken or not found>");

    // se pa da pra jogar no tauri isso aqui ja
    let mut instanciadobotircmuitofoda = bot::Irc::new(config).await;

    // remover await e proseguir com o codigo se nao ele so destroi a instancia do bot
    instanciadobotircmuitofoda.run(commandshandler).await;

    Ok(())
}
