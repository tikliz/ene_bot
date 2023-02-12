use futures::prelude::*;
use irc::client::{prelude::*, ClientStream};

pub struct Irc {
    pub config: Config,
    pub client: Client,
    pub stream: ClientStream,
    pub sender: Sender,
}
impl Irc {
    pub async fn new(config: Config) -> Self {
        loop {
            println!("<info: connecting>");

            // talvez fazer com reclusividade pra remover esse clone aqui
            let mut aclient = Client::from_config(config.clone()).await.unwrap();
            aclient.identify().unwrap();

            let mut astream = aclient.stream().unwrap();

            if astream.next().await.is_none() {
                //println!("<info: connection failed>");
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                continue;
            }

            let asender = aclient.sender();

            println!("<info: connected as \"{}\">", aclient.current_nickname());

            return Self {
                config: config,
                client: aclient,
                stream: astream,
                sender: asender,
            };
        }
    }

    pub async fn run(&mut self, commandhandler: fn(irc: &mut Irc, target: &String, msg: &String)) {
        loop {
            let message = match self.stream.next().await.transpose() {
                Ok(v) => v,
                Err(e) => panic!("5 unwrap resulted in {}", e),
            };
            if message.is_none() {
                println!("<info: disconnected>");

                // precisa inserir magia negra pra ele tentar se reconnectar sozinho
                //self = &Irc::new(self.config).await;

                break;
            }
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
                    commandhandler(self, target, msg);
                }
                _ => (),
            }
        }
    }


    pub async fn stop(&self) {
        // fazer uma forma de fazer o run parar
    }

    // criar um destructor para
    // self.client.send_quit("").unwrap();

}
