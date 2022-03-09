use std::{net::TcpStream, io::{Read, BufWriter, Write}};
use thirtyfour::{WebDriver, DesiredCapabilities};
use serde::{self, Deserialize, Serialize};
use crate::{error::AppError, ClientRequest, ServerResponse, scrapper::{search_keyword, find_anime_info}};

#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeInfo {
    pub imgsrc: String,
    pub name: String,
    pub genre: String,
    pub episodes: Vec<EpisodeInfo>,
    pub description: String,
    pub released: String,
    pub status: String,
}

impl AnimeInfo {
    pub fn new() -> Self {
        AnimeInfo {
            imgsrc: String::new(),
            name: String::new(),
            genre: String::new(),
            episodes: Vec::new(),
            description: String::new(),
            released: String::new(),
            status: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EpisodeInfo {
    pub episode: String,
    pub link: String,
}

impl EpisodeInfo {
    pub fn new() -> Self {
        EpisodeInfo { episode: String::new(), link: String::new() }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AnimeList {
    pub name: String,
    pub link: String,
    pub imgsrc: String,
    pub releasedate: String,
}

impl AnimeList {
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn link(&mut self, link: &str) -> &mut Self {
        self.link = link.to_string();
        self
    }

    pub fn releasedate(&mut self, date: String) -> &mut Self {
        self.releasedate = date;
        self
    }

    pub fn imgsrc(&mut self, src: String) -> &mut Self {
        self.imgsrc = src;
        self 
    }
}


//the real handler starts here

pub async fn handle_client(stream: TcpStream) -> Result<(), AppError> {
    tokio::spawn(async move {
        loop {

            let mut stream = stream.try_clone().map_err(|_e| AppError::TcpStreamCloneErr).unwrap();
            let mut data = [0 as u8; 10000];
            match stream.read(&mut data) {
                Ok(_size) => {
                    let mut caps = DesiredCapabilities::chrome();
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
                    ).map_err(|_e| AppError::ChromeOptionErr).unwrap();
                    let driver = WebDriver::new("http://localhost:9515", &caps)
                    .await
                    .map_err(|_e| AppError::CreateWebDriverErr)
                    .unwrap();
                    let request = eliminate_zero(data).unwrap();
                    let deserialized_request: ClientRequest = serde_json::from_str(&request)
                    .map_err(|_e| AppError::DeserializeErr)
                    .unwrap();

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
    Ok(())
}

fn eliminate_zero(data: [u8; 10000]) -> Result<String, AppError> {
    let mut new_data: Vec<u8> = Vec::new();
    for i in data {
        if i == 0 {
            break;
        } else {
            new_data.push(i);
        }
    }
    Ok(String::from_utf8(new_data).unwrap())
}

async fn process_request(request: ClientRequest, driver: WebDriver) -> ServerResponse {
    let mut _server_response: ServerResponse = ServerResponse::None;
    match request {
        ClientRequest::Anime(s) => {
            match find_anime_info(driver, s).await {
                Ok(t) => {
                    let response = ServerResponse::AnimeInfo(t);
                    _server_response = response;
                },
                Err(e) => {
                    let response = ServerResponse::Err(e);
                    _server_response = response;
                },
            } 
        },
        ClientRequest::Search(s) => {
            match search_keyword(s, driver).await {
                Ok(t) => {
                    let response = ServerResponse::AnimeSearch(t);
                    _server_response = response;
                },
                Err(e) => {
                    let response = ServerResponse::Err(e);
                    _server_response = response;
                },
            } 
        },
    }
    return _server_response
}

async fn send_response(response: ServerResponse, stream: &TcpStream) {
    let serialized_response = serde_json::to_string(&response)
    .map_err(|_e| AppError::SerializeErr)
    .unwrap();

    let mut writer = BufWriter::new(stream);
    writer.write_all(serialized_response.as_bytes())
    .map_err(|_e| AppError::WriteErr)
    .unwrap();
    writer.flush()
    .map_err(|_e| AppError::FlushErr)
    .unwrap();
    println!("After write");
}
