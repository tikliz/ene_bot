use irc::client::{Client};

pub fn run(client: &Client) -> Option<String> {
    client.send_quit("").unwrap();
    None

}