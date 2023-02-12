use futures::prelude::*;
use irc::client::{prelude::*, ClientStream};

pub async fn connect_client() -> (Option<Client>, Option<ClientStream>) {
    let config = Config {
        nickname: Some("rheniit".to_owned()),
        server: Some("irc.chat.twitch.tv".to_owned()),
        port: Some(6697.to_owned()),
        channels: vec!["#rheniit".to_owned()],
        password: Some("oauth:u8wve6dyl112zhvfz3fzscqzcvr61h".to_owned()),
        ..Default::default()
    };

    // let path = PathBuf::from("./config.toml");
    // let config = Config::load(path)?;


    let mut client = Client::from_config(config).await.unwrap();


    client.identify().unwrap();
    let sender = client.sender();
    let mut stream = client.stream().unwrap();
    
    if stream.next().await.is_none() {
        println!("FAILED TO CONNECT (i think)!");
        //irccommands::leave::run(&client);
        //sleep(Duration::from_secs(5)).await;
        return (None, None)

    }

    (Some(client), Some(stream))
}