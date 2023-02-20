use std::fs;

use futures::executor::block_on;
use serde::{Serialize, Serializer, ser::SerializeStruct};
use serde_json::Value;
use tokio::{runtime::Runtime, task};
use reqwest::{self, get};
use tauri::{window, Window, Manager, regex};
use url::Url;
use scraper::{Html, Selector};
use crate::bot;

#[derive(Clone)]
struct Payload {
    url: String,
    map_artist: String,
    map_title: String,
    mapper: String,
    map_bpm: String,
    map_bg: String,
    requester: String,

}
impl Serialize for Payload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
            let mut state = serializer.serialize_struct("Color", 3)?;
            state.serialize_field("url", &self.url)?;
            state.serialize_field("map_artist", &self.map_artist)?;
            state.serialize_field("map_title", &self.map_title)?;
            state.serialize_field("mapper", &self.mapper)?;
            state.serialize_field("map_bpm", &self.map_bpm)?;
            state.serialize_field("map_bg", &self.map_bg)?;
            state.serialize_field("requester", &self.requester)?;
            state.end()
        
    }

}

impl IntoIterator for Payload {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            self.url,
            self.map_artist, 
            self.map_title,
            self.mapper,
            self.map_bpm,
            self.map_bg,
            self.requester,
            
            ].into_iter()

    }

}

fn is_valid_url(url_str: &str) -> bool {
    match Url::parse(url_str) {
        Ok(_) => true,
        Err(_) => false,
    }
}

async fn create_payload(url: String, sent_by: String, window: Window) {
    println!("<task: parsing request!>");
    let response = get(url.clone()).await.unwrap();

    // parse the HTML response
    let body = response.text().await.unwrap();
    let document = Html::parse_document(&body);

    // extract json
    let selector = Selector::parse("#json-beatmapset").unwrap();
    let script = document.select(&selector).next().unwrap().inner_html();
    let json: Value = serde_json::from_str(&script).unwrap();
    
    // println!("Artist: {}", json["artist"]);
    // println!("Title: {}", json["title"]);
    // println!("Creator: {}", json["creator"]);
    // // println!("id: {}", json["id"]);
    // // println!("NSFW: {}", json["nsfw"]);
    // println!("BPM: {}", json["bpm"]);
    // // println!("noms: {}", json["nominations_summary"]);
    // println!("bg: {}", json["covers"]["cover"]);
    // // println!("bg 2x: {}", json["covers"]["cover@2x"]);

    // map_artist: String,
    // map_title: String,
    // mapper: String,
    // map_bpm: String,
    // map_bg: String,
    // requester: String,
    let payload = Payload {
        url,
        map_artist: json["artist"].to_string().replace("\"", ""),
        map_title: json["title"].to_string().replace("\"", ""),
        mapper: json["creator"].to_string().replace("\"", ""),
        map_bpm: json["bpm"].to_string().replace("\"", ""),
        map_bg: json["covers"]["cover"].to_string().replace("\"", ""),
        requester: sent_by,

    };
    
    // for i in payload.clone() {
    //     println!("{}", i);

    // };
    window.emit_all("ADD_TO_LIST", payload).unwrap();

}

pub fn run(bot: &mut bot::Irc, handler: &Vec<bot::CommandRegister>, target: &String, sent_by: Option<String>, msg: Option<&String>, window: Option<&Window>) -> Option<String> {
    // precisa dar um jeito de saber quem mandou o commando
    //bot.sender;
    
    if let Some(msg) =  msg{
        
        let args: Vec<&str> = msg.splitn(2, ' ').collect();
        
        //args.count();
        if !is_valid_url(args[0]) {
            return Some("Invalid argument, this is not a valid URL + help".to_string());

        }
        if args[0].contains("https://osu.ppy.sh/beatmapsets/") {
            //window.unwrap();
            task::spawn(create_payload(args[0].to_string(), sent_by.unwrap(), window.unwrap().clone()));

        } else {
            return Some("Not an osu! request, maybe valid.".to_string());

        }
        
        return Some("Done!".to_string());

        // if args.len() == 2 {
        //     if !is_valid_url(args[1]) {
        //         return Some("Invalid argument, this is not a valid URL  + help".to_string());
        //     }

        //     // aqui adicionar link e quem mandou em um vector struct

        //     return Some("Done!".to_string());
        // }
        // return Some("Invalid number of arguments + help".to_string());

    } // else
    Some("Requests are closed!".to_string())
}
