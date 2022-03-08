mod scrapper;
mod handler;

use std::{net::TcpListener};
use serde::{self, Deserialize, Serialize};
use handler::{AnimeList, handle_client, AnimeInfo};
use tokio;
use thirtyfour::{self, prelude::WebDriverResult};

#[derive(Debug, Deserialize, Serialize)]
enum ClientRequest {
    Search(String),
    Anime(AnimeList),
}

#[derive(Debug, Deserialize, Serialize)]
enum ServerResponse {
    AnimeSearch(Vec<AnimeList>),
    AnimeInfo(AnimeInfo),
    Err,
}

#[tokio::main]
async fn main() -> WebDriverResult<()>  {
    //test for find anime info
    /* 
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", &caps).await.unwrap();
    let anime = AnimeList {
        name: String::from("Hakuouki"),
        link: String::from("/category/hakuouki"),
        imgsrc: String::from("https://gogocdn.net/cover/hakuouki.png"),
        releasedate: String::from("Released: 2010"),
    };

    let result = find_anime_info(driver, anime).await;
    */

    /* 
    //test for finding animelist
    let keyword = "isekai".to_string();
    let mut caps = DesiredCapabilities::chrome();
    //caps.set_headless().unwrap();
    caps.add_chrome_option(
        "prefs",
        serde_json::json!({
            "profile.default_content_settings": {
                "images": 2
            },
            "profile.managed_default_content_settings": {
                "images": 2
            }
        }),
    ).unwrap();
    let driver = WebDriver::new("http://localhost:9515", &caps).await.unwrap();
    let result = search_keyword(keyword, driver).await.unwrap();
    */


    //create server for front end
    let listener = TcpListener::bind("127.0.0.1:4040").unwrap();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_client(stream).await
        }
    }
    Ok(())
}