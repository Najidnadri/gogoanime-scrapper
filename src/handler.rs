use std::{net::TcpStream, io::{Read, BufWriter, Write}};

use thirtyfour::{WebDriver, By, DesiredCapabilities};
use serde::{self, Deserialize, Serialize};

use crate::{scrapper::{find_name_link, release_date, img_src, href_link}, ServerResponse, ClientRequest};

pub const BASE_URL: &str = "https://gogoanime.fan";

#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeInfo {
    imgsrc: String,
    name: String,
    genre: String,
    episodes: Vec<EpisodeInfo>,
    description: String,
    released: String,
    status: String,
}

impl AnimeInfo {
    fn new() -> Self {
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
    episode: String,
    link: String,
}

impl EpisodeInfo {
    fn new() -> Self {
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

pub async fn search_keyword(keyword: String, driver: WebDriver) -> Result<Vec<AnimeList>, Box<dyn std::error::Error>> {
    //data
    let mut anime_list: Vec<AnimeList> = vec![];

    let keyword = keyword.replace(" ", "%20");
    let url = format!("https://gogoanime.fan//search.html?keyword={}", keyword);
     
    //navigate to gogoanime website
    driver.get(url).await.expect("error in navigating to website");
/*
    //find pages
    let page_element = driver.find_element(By::ClassName("pagination-list")).await.expect("error searching for page");
    let page_list = page_element.find_elements(By::Tag("li")).await.expect("error searching for list tag");
    let pages = page_list.len();
    let mut current_page: usize = 1;
*/
    
    //go into items
    let items = driver.find_element(By::ClassName("items")).await.expect("error searching for item list");

    //go into each list
    let list = items.find_elements(By::Tag("li")).await.expect("cannot find list tag");
    
    for info in list {
        let mut anime = AnimeList::default();

        //name & link
        let name_link = find_name_link(info.clone()).await.expect("error finding name or link");
        anime.name(&name_link[0]);
        anime.link(&name_link[1]);

        //release date
        anime.releasedate(release_date(info.clone()).await.expect("error finding release date"));

        //imgsrc
        anime.imgsrc(img_src(info).await.expect("error finding imgsrc"));

        anime_list.push(anime);
    }

    driver.quit().await.unwrap();
    println!("{:#?}", anime_list);

    //println!("{:#?}", anime_list);

    Ok(anime_list)
}

pub async fn find_anime_info(driver: WebDriver, anime: AnimeList) -> AnimeInfo {
    let url = format!("{}{}", BASE_URL, anime.link);
    let mut anime_info: AnimeInfo = AnimeInfo::new();

    //navigate to gogoanime website
    driver.get(url).await.expect("error in navigating to website");

    //imgsrc
    anime_info.imgsrc = anime.imgsrc;

    //name
    anime_info.name = anime.name;

    //released
    anime_info.released = anime.releasedate;

    //description
    let anime_info_body = driver.find_element(By::ClassName("anime_info_body_bg")).await.expect("cannot find anime info body");
    let class_type = anime_info_body.find_elements(By::ClassName("type")).await.expect("cant find type class");

    for i in class_type {
        let mut element_text = i.text().await.unwrap();
        if element_text.contains("Plot Summary:") {
            element_text.drain(..14);
            anime_info.description = element_text;
        } else if element_text.contains("Genre:") {
            element_text.drain(..7);
            anime_info.genre = element_text;
        } else if element_text.contains("Status") {
            element_text.drain(..8);
            anime_info.status = element_text;
        }
    }

    //episodes
    let episodes_element = driver.find_element(By::Id("episode_related")).await.unwrap();
    let episodes_list = episodes_element.find_elements(By::Tag("li")).await.unwrap();

    let mut episodes = Vec::new();

    for i in episodes_list {
        let mut episode = EpisodeInfo::new();
        let html = i.inner_html().await.unwrap();
        let episode_link = href_link(html);
        episode.link = episode_link;

        //find which episode
        let name_class = i.find_element(By::ClassName("name")).await.unwrap();
        episode.episode = name_class.text().await.unwrap();
        episodes.push(episode);
    }
    anime_info.episodes = episodes;

    //println!("{:#?}", anime_info);

    driver.quit().await.expect("error quitting the driver");

    anime_info
}


//the real handler starts here

pub async fn handle_client(stream: TcpStream) {
    tokio::spawn(async move {
        loop {

            let mut stream = stream.try_clone().unwrap();
            let mut data = [0 as u8; 5000];
            match stream.read(&mut data) {
                Ok(_size) => {
                    let caps = DesiredCapabilities::chrome();
                    let driver = WebDriver::new("http://localhost:9515", &caps).await.unwrap();
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
    let mut _server_response = ServerResponse::Err;
    match request {
        ClientRequest::Anime(s) => {
            let anime_info = find_anime_info(driver, s).await;
            let response = ServerResponse::AnimeInfo(anime_info);
            _server_response = response
        },
        ClientRequest::Search(s) => {
            let anime_list = search_keyword(s, driver).await.unwrap();
            let response = ServerResponse::AnimeSearch(anime_list);
            _server_response = response
        },
    }
    return _server_response
}

async fn send_response(response: ServerResponse, stream: &TcpStream) {
    let serialized_response = serde_json::to_string(&response).unwrap();

    let mut writer = BufWriter::new(stream);
    writer.write_all(serialized_response.as_bytes()).expect("could not write");
    writer.flush().expect("cannot flush");
    println!("After write");
}
