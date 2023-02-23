use futures::prelude::*;
use irc::client::{prelude::*, ClientStream};
use tauri::Window;
use tokio::time::error::Elapsed;

use crate::irccommands::help;
pub struct Irc {
    pub config: Config,
    pub client: Client,
    pub stream: ClientStream,
    pub sender: Sender,
    pub run_bot: bool,
}



impl Irc {
    pub async fn new(config: Config) -> Self {
        async fn test(config: Config, dur: u64) -> Option<(Client, ClientStream)>
        {
            println!("<info: connecting>");    
        
            let mut aclient = Client::from_config(config).await.unwrap();
            aclient.identify().unwrap();
            // request more info from twitch
            aclient.send("CAP REQ :twitch.tv/tags\r\n").unwrap();

            let mut astream = aclient.stream().unwrap();

            if astream.next().await.is_none() {
                println!("<info: connection failed>");
                tokio::time::sleep(tokio::time::Duration::from_secs(dur)).await;
                return None;
            };
            Some((aclient, astream))
        }
        
        let mut test_option: Option<(Client, ClientStream)>;
        let mut dur: u64 = 1;
        loop
        {
            test_option = test(config.clone(), dur).await;
            if test_option.is_some()
            {
                break;
            }
            if dur <= 60
            {
                dur += dur;
            }
        }
       
        let mut aclient: Client;
        let mut astream: ClientStream;
        match test_option {
            Some((client, stream)) => {aclient = client; astream = stream},
            _ => panic!("silly mode activated")

        }
        let asender = aclient.sender();

        println!("<info: connected as \"{}\">", aclient.current_nickname());

        return Self {
            config: config,
            client: aclient,
            stream: astream,
            sender: asender,
            run_bot: true,
        };
    }

    pub async fn run(&mut self, commandhandler: &Handler) -> bool {
        while self.run_bot {
            let message = match self.stream.next().await.transpose() {
                Ok(v) => v,
                Err(e) => panic!("<fatal: something went wrong with the irc stream, but i was not disconnected. error was {}>", e),
            };
            if message.is_some() {
                // precisa inserir magia negra pra ele tentar se reconnectar sozinho
                //self = &Irc::new(self.config).await;

                match message.as_ref().unwrap().command {
                    Command::PING(ref _target, ref _msg) => {
                        println!("<info: ping received>");
                    }
                    Command::PONG(ref _target, ref _msg) => {
                        println!("<info: pong received>");
                    }
                    Command::NOTICE(ref _target, ref msg) => {
                        println!("<info: notice received \"{}\">", msg);
                    }
                    Command::PRIVMSG(ref target, ref msg) => {
                        //&self, bot: &mut Irc, target: &String, msg: String
                        // sender nickname (pretty scuffed, need to fix)
                        let mut sent_by: Option<String> = None;
                        let mut tag: Option<String> = None;
                        // println!("test");
                        if let Some(nick) = message.clone().unwrap().prefix.unwrap().to_string().split('!').next() {
                            //println!("uga");
                            sent_by = Some(nick.to_string());
                            
                        }
                        if let Some(opt_tag) = message.clone().unwrap().tags {
                            //println!("checking: {:?}", opt_tag);
                            if opt_tag[3].1 == Some("5bb0c951-4b00-4ade-8750-7aa8d79520e9".to_string()) {
                                tag = Some("REQ".to_string());
                            
                            }
                        
                        }
                        
                        commandhandler.run(self, target, tag, sent_by, msg).await;

                    }
                    _ => (),
                }
                
            };
            // else {
            // panic!("<info: disconnected>");
            // não aconteceu ainda, mas quando acontecer milagrosamente reconecte pls
            //break;


        }
        println!("<info: stopped running>");
        self.run_bot

    }

    pub async fn stop(&mut self) {
        self.run_bot = false;
        self.client.send_quit("").unwrap();
        
    }

}

pub fn str_to_option(s: &String) -> Option<&String> {
    if s == " " {
        return None;
    }
    Some(s)
}

#[derive(Clone)]
pub struct CommandRegister {
    pub window: Option<Window>,
    pub command: Option<String>,
    pub tag: Option<String>,
    pub description: String,
    pub usage: String,
    pub run: fn(
        bot: &mut Irc,
        handler: &Vec<CommandRegister>,
        target: &String,
        tag: Option<String>,
        sent_by: Option<String>,
        msg: Option<&String>,
        window: Option<&Window>,
    ) -> Option<String>,
}

pub struct Handler {
    pub commands: Vec<CommandRegister>, //run: fn(),
}
impl Handler {
    pub fn new(commands: Vec<CommandRegister>) -> Self {
        Self { commands }
    }

    pub async fn run(&self, bot: &mut Irc, target: &String, tag: Option<String>, sent_by: Option<String>, msg: &str) {
        let mut split_msg = msg.splitn(2, ' ').collect::<Vec<&str>>();
        if split_msg.len() < 2 {
            split_msg.push(" ")

        }

        
        
        for register in &self.commands {
            if tag.is_some() {
                println!("TAG EXISTS");
                // match tag.unwrap().as_str() {
                // "REQ" => println!("REQUEST FROM CHANNEL POINTS"),
                //     _ => println!("IDK"),
    
                // }
                if register.tag == tag {
                    if let Some(response) = (register.run)(bot, &self.commands, target, tag, sent_by, str_to_option(&split_msg[0].to_string()), register.window.as_ref()) {
                        let send_response = bot.sender.send_privmsg(target, &response);
                        match send_response {
                            Ok(()) => {
                                println!("<info: response sent \"{}\">", response);
                            }
                            Err(_) => {
                                println!(
                                    "<error: failed to send response \"{:#?}\">",
                                    Some(send_response)
                                );
                            }
                        };
                    }

                }
                return;
    
            }
            
            if Some(split_msg[0].to_string()) == register.command {
                if register.command == Some("!q".to_string()) {
                    bot.stop().await;
                    return;

                }
                
                if let Some(response) = (register.run)(bot, &self.commands, target, tag, sent_by, str_to_option(&split_msg[1].to_string()), register.window.as_ref()) {
                    let send_response = bot.sender.send_privmsg(target, &response);
                    match send_response {
                        Ok(()) => {
                            println!("<info: response sent \"{}\">", response);
                        }
                        Err(_) => {
                            println!(
                                "<error: failed to send response \"{:#?}\">",
                                Some(send_response)
                            );
                        }
                    };
                }
                return;

            }
        }
    }
}