use futures::prelude::*;
use irc::client::{prelude::*, ClientStream};
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
    
    // da pra fazer ela dentro acho uhmm vdd 
    async fn asdjhasgd()
    {

        
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

                match message.unwrap().command {
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
                        commandhandler.run(self, target, msg).await;
                    }
                    _ => (),
                }
                
            } else {
            panic!("<info: disconnected>");
            // não aconteceu ainda, mas quando acontecer milagrosamente reconecte pls
            //break;

            }
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
    pub command: String,
    pub description: String,
    pub usage: String,
    pub run: fn(
        bot: &mut Irc,
        handler: &Vec<CommandRegister>,
        target: &String,
        msg: Option<&String>,
    ) -> Option<String>,
}

pub struct Handler {
    pub commands: Vec<CommandRegister>, //run: fn(),
}
impl Handler {
    pub fn new(commands: Vec<CommandRegister>) -> Self {
        Self { commands: commands }
    }

    pub async fn run(&self, bot: &mut Irc, target: &String, msg: &str) {
        let mut split_msg = msg.splitn(2, ' ').collect::<Vec<&str>>();
        if split_msg.len() < 2 {
            split_msg.push(" ")

        }
        
        for register in &self.commands {
            if split_msg[0] == register.command {
                if register.command == "!q" {
                    bot.stop().await;
                    break;
                }
                
                if let Some(response) = (register.run)(bot, &self.commands, target, str_to_option(&split_msg[1].to_string())) {
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
                break;
            }
        }
    }
}