mod scrapper;
mod handler;

use std::{net::{TcpListener, TcpStream}, io::{Read, BufWriter, Write}};
use serde_json;
use serde::{self, Deserialize, Serialize};
use handler::{AnimeList, search_keyword};
use tokio;
use thirtyfour::{self, prelude::WebDriverResult, DesiredCapabilities, WebDriver};

#[derive(Debug, Deserialize, Serialize)]
enum ClientRequest {
    Search(String),
    Anime(String),
}

#[derive(Debug, Deserialize, Serialize)]
enum ServerResponse {
    AnimeSearch(Vec<AnimeList>),
    AnimeInfo(Anime),
    Err,
}

#[derive(Debug, Deserialize, Serialize)]
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
    Ok(())
}

async fn handle_client(stream: TcpStream) {
    tokio::spawn(async move {
        loop {
            let caps = DesiredCapabilities::chrome();
            let driver = WebDriver::new("http://localhost:9515", &caps).await.unwrap();

            let mut stream = stream.try_clone().unwrap();
            let mut data = [0 as u8; 5000];
            match stream.read(&mut data) {
                Ok(size) => {
                    let request = eliminate_zero(data);
                    let deserialized_request: ClientRequest = serde_json::from_str(&request).unwrap();
                    let response = process_request(deserialized_request, driver).await;
                    send_response(response, &stream).await;
                }
                Err(_) => {
                    println!("error in handle client");
                    break;
                }
            }


        }
    });
}

fn eliminate_zero(data: [u8; 5000]) -> String {
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

async fn process_request(request: ClientRequest, driver: WebDriver) -> ServerResponse {
    let mut server_response = ServerResponse::Err;
    match request {
        ClientRequest::Anime(_s) => {
            server_response = ServerResponse::Err
        },
        ClientRequest::Search(s) => {
            let anime_list = search_keyword(s, driver).await.unwrap();
            let response = ServerResponse::AnimeSearch(anime_list);
            server_response = response
        },
    }
    return server_response
}

async fn send_response(response: ServerResponse, stream: &TcpStream) {
    let serialized_response = serde_json::to_string(&response).unwrap();

    let mut writer = BufWriter::new(stream);
    writer.write_all(serialized_response.as_bytes()).expect("could not write");
    writer.flush().expect("cannot flush");
    println!("After write");
}