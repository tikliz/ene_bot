use tauri::{window, Window, Manager};
use url::Url;
use crate::bot;

fn is_valid_url(url_str: &str) -> bool {
    match Url::parse(url_str) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn run(bot: &mut bot::Irc, handler: &Vec<bot::CommandRegister>, target: &String, msg: Option<&String>, window: Option<&Window>) -> Option<String> {
    // precisa dar um jeito de saber quem mandou o commando
    //bot.sender;
    
    {
        
        let args: Vec<&str> = msg.unwrap().splitn(2, ' ').collect();
        
        //args.count();
        if !is_valid_url(args[0]) {
            return Some("Invalid argument, this is not a valid URL  + help".to_string());

        }
        window.unwrap().emit_all("ADD_TO_LIST", args[0]).unwrap();
        return Some("Done!".to_string());

        // if args.len() == 2 {
        //     if !is_valid_url(args[1]) {
        //         return Some("Invalid argument, this is not a valid URL  + help".to_string());
        //     }

        //     // aqui adicionar link e quem mandou em um vector struct

        //     return Some("Done!".to_string());
        // }
        // return Some("Invalid number of arguments + help".to_string());
    }
    Some("Requests are closed!".to_string())
}
