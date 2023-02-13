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
    // talvez passar essas coisas por variavel de ambiente e a forma mais segura de se fazer isso e amais comum de ver em codigo alheio
    let config = Config::load(PathBuf::from("./../../config.toml"))
        .expect("<fatal: config file is broken or not found>");

    // se pa da pra jogar no tauri isso aqui ja
    let mut instanciadobotircmuitofoda = bot::Irc::new(config).await;

    
    let mut handler_foda = bot::Handler::new([
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
    ]
    .to_vec());
    
    // remover await e proseguir com o codigo se nao ele so destroi a instancia do bot
    
    //&self, bot: &mut Irc, target: &String, msg: &String

    instanciadobotircmuitofoda.run(handler_foda).await;

    Ok(())
}
