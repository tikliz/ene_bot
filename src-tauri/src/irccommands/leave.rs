use tauri::Window;

use crate::bot;

pub fn run(bot: &mut bot::Irc, handler: &Vec<bot::CommandRegister>, target: &String, tag: Option<String>, sent_by: Option<String>,  msg: Option<&String>,  window: Option<&Window>) -> Option<String> {
{
    // if split_msg[0].unwrap() == "!help"
    // {

    // self.commands[1].command;
    // self.commands[1].description;
    // self.commands[1].usage;

    // let args = msg.split_whitespace().collect::<Vec<&str>>();
    // if args.len() == 1 {
    //     return Some("full help message".to_string());
    // }
    // if args.len() == 2 {
    //     return Some("especific help message".to_string());
    // }
    // Some("full help message com aviso de erro de argumento".to_string())
    if msg.is_some() {
        return Some("full help message".to_string())

    }
    Some("general help message".to_string())

}}