use std::fs;

use futures::executor::block_on;
use serde_json::Value;
use tokio::{runtime::Runtime, task};
use reqwest::{self, get};
use tauri::{window, Window, Manager, regex};
use url::Url;
use scraper::{Html, Selector};
use crate::bot;

struct Payload {
    map_bg: String,
    map_title: String,
    map_artist: String,
    mapper: String,
    requester: String,

}

fn is_valid_url(url_str: &str) -> bool {
    match Url::parse(url_str) {
        Ok(_) => true,
        Err(_) => false,
    }
}

async fn create_payload(url: String) {
    println!("<parsing request!>");
    let response = get(url).await.unwrap();

    // parse the HTML response
    let body = response.text().await.unwrap();
    let document = Html::parse_document(&body);

    // extract json
    let selector = Selector::parse("#json-beatmapset").unwrap();
    let script = document.select(&selector).next().unwrap().inner_html();
    let json: Value = serde_json::from_str(&script).unwrap();
    
    println!("Artist: {}", json["artist"]);
    println!("Title: {}", json["title"]);
    println!("Creator: {}", json["creator"]);
    println!("id: {}", json["id"]);
    println!("NSFW: {}", json["nsfw"]);
    println!("BPM: {}", json["bpm"]);
    println!("noms?: {}", json["nominations_summary"]);
    println!("bg: {}", json["covers"]["cover"]);
    // println!("bg 2x: {}", json["covers"]["cover@2x"]);

}

pub fn run(bot: &mut bot::Irc, handler: &Vec<bot::CommandRegister>, target: &String, msg: Option<&String>, window: Option<&Window>) -> Option<String> {
    // precisa dar um jeito de saber quem mandou o commando
    //bot.sender;
    
    if msg.is_some(){
        
        let args: Vec<&str> = msg.unwrap().splitn(2, ' ').collect();
        
        //args.count();
        if !is_valid_url(args[0]) {
            return Some("Invalid argument, this is not a valid URL  + help".to_string());

        }
        if args[0].contains("https://osu.ppy.sh/beatmapsets/") {
            task::spawn(create_payload(args[0].to_string()));

        } else {
            return Some("Not an osu! request, maybe valid.".to_string());

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
