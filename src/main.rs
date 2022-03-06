mod scrapper;
mod handler;

use std::{net::{TcpListener, TcpStream}, io::Read};

use handler::AnimeList;
use tokio;
use thirtyfour::{self, prelude::WebDriverResult, DesiredCapabilities, WebDriver};
use crate::handler::scrap_now;

enum ClientRequest {
    Search(String),
    Anime(String),
}

enum ServerResponse {
    AnimeList(Vec<AnimeList>),
    AnimeInfo(Anime)
}

struct Anime {
    name: String,
    release_date: String,
    img_src: String,
    description: String,
    episodes: usize,
    status: String,
    genre: String,
}

#[tokio::main]
async fn main() -> WebDriverResult<()>  {

    //create server for front end
    let listener = TcpListener::bind("127.0.0.1:4040").unwrap();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_client(stream).await
        }
    }

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", &caps).await?;

    let demo_search = "ova".to_string();

    //start scrapping gogoanime
    let url = "https://gogoanime.fan//search.html?keyword=ova".to_string();
    let driver = scrap_now(url, driver).await.expect("error in scraping");

    driver.quit().await.expect("error while quitting webdriver");
    Ok(())
}

async fn handle_client(stream: TcpStream) {
    tokio::spawn(async move {
        loop {
            let mut stream = stream.try_clone().unwrap();
            let mut data = [0 as u8; 1000];
            match stream.read(&mut data) {
                Ok(size) => {
                    let request = eliminate_zero(data);
                    println!("{}", request);
                }
                Err(_) => {
                    println!("error in handle client");
                }
            }
        }
    });
}

fn eliminate_zero(data: [u8; 1000]) -> String {
    let mut new_data: Vec<u8> = Vec::new();
    for i in data {
        if i == 0 {
            break;
        } else {
            new_data.push(i);
        }
    }
    String::from_utf8(new_data).unwrap()
}